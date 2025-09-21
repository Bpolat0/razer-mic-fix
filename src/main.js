const { invoke } = window.__TAURI__.core;

const $ = (s) => document.querySelector(s);
const vol = $("#vol"), vtxt = $("#volv");
const intv = $("#intv"), itxt = $("#intvv");
const devSel = $("#device");
const langSel = $("#language");
const btnStart = $("#start"), btnStop = $("#stop"), btnExit = $("#exit"), btnHide = $("#hide-to-tray");
const chkAuto = $("#autostart"), chkStartMonitoring = $("#start-monitoring-launch");

let currentTranslations = {};

async function saveSettings() {
  const cfg = { 
    vol: +vol.value, 
    intv: +intv.value, 
    dev: devSel.value,
    language: langSel.value,
    autostart: chkAuto.checked,
    startMonitoring: chkStartMonitoring.checked
  };
  
  try {
    await invoke("save_settings", { settings: cfg });
  } catch (error) {
    console.error("Error saving settings:", error);
  }
}

async function loadSettings() {
  try {
    const cfg = await invoke("load_settings");
    if (cfg.vol != null) vol.value = cfg.vol;
    if (cfg.intv != null) intv.value = cfg.intv;
    if (cfg.dev != null) devSel.value = cfg.dev;
    if (cfg.language != null) langSel.value = cfg.language;
    if (cfg.autostart != null) chkAuto.checked = cfg.autostart;
    if (cfg.startMonitoring != null) chkStartMonitoring.checked = cfg.startMonitoring;
    
    vtxt.textContent = `Target: ${vol.value}%`;
    itxt.textContent = `${intv.value} s`;
  } catch (error) {
    console.error("Error loading settings:", error);
  }
}

async function loadLanguages() {
  try {
    const languages = await invoke("get_available_languages");
    langSel.innerHTML = "";
    languages.forEach(lang => {
      const option = document.createElement("option");
      option.value = lang.code;
      option.textContent = lang.name;
      langSel.appendChild(option);
    });
  } catch (error) {
    console.error("Error loading languages:", error);
  }
}

async function loadTranslations(lang) {
  try {
    currentTranslations = await invoke("get_translations", { language: lang });
    updateUI();
  } catch (error) {
    console.error("Error loading translations:", error);
  }
}

function updateUI() {
  if (!currentTranslations) return;
  
  const elements = document.querySelectorAll("[data-i18n]");
  elements.forEach(el => {
    const key = el.getAttribute("data-i18n");
    if (currentTranslations[key]) {
      if (el.tagName === "OPTION") {
        el.textContent = currentTranslations[key];
      } else {
        el.textContent = currentTranslations[key];
      }
    }
  });
  
  // Update volume and interval texts
  vtxt.textContent = `Target: ${vol.value}%`;
  itxt.textContent = `${intv.value} ms`;
}

async function refreshDevices() {
  console.log("refreshDevices called");
  devSel.innerHTML = "";
  const optDef = document.createElement("option");
  optDef.value = ""; 
  optDef.textContent = currentTranslations.default_windows || "Default (Windows)";
  devSel.appendChild(optDef);
  
  try {
    console.log("Calling list_devices...");
    const list = await invoke("list_devices");
    console.log("Received devices:", list);
    (list || []).forEach(d => {
      console.log("Adding device:", d);
      const o = document.createElement("option");
      o.value = d.id; o.textContent = d.name; devSel.appendChild(o);
    });
  } catch (error) {
    console.error("Error loading devices:", error);
  }
}

async function hideToTray() {
  try {
    // Backend komutu ile pencereyi gizle
    await invoke("hide_window");
    
    // Sonra notification göster
    await invoke("show_notification", { 
      title: currentTranslations.app_title || "Razer Mic Fix",
      body: currentTranslations.tray_notification || "Application hidden to system tray"
    });
  } catch (error) {
    console.error("Error hiding to tray:", error);
  }
}

async function init() {
  // Prevent any accidental window shows during init
  console.log("Initializing app...");
  
  // Load languages first
  await loadLanguages();
  
  // Load settings
  await loadSettings();
  
  // Load translations based on saved language
  await loadTranslations(langSel.value || "en");
  
  vtxt.textContent = `Target: ${vol.value}%`;
  itxt.textContent = `${intv.value} s`;
  
  // Event listeners
  vol.oninput = () => { 
    vtxt.textContent = `Target: ${vol.value}%`; 
    saveSettings(); 
  };
  
  intv.oninput = () => { 
    itxt.textContent = `${intv.value} s`; 
    saveSettings(); 
  };
  
  devSel.onchange = saveSettings;
  
  langSel.onchange = async () => {
    // Show loading indicator
    const originalText = langSel.options[langSel.selectedIndex].text;
    langSel.disabled = true;
    
    try {
      await loadTranslations(langSel.value);
      await saveSettings();
      
      // Update only the "Default" option text without full refresh
      const defaultOption = devSel.querySelector('option[value=""]');
      if (defaultOption) {
        defaultOption.textContent = currentTranslations.default_windows || "Default (Windows)";
      }
    } catch (error) {
      console.error("Language change error:", error);
    } finally {
      langSel.disabled = false;
    }
  };
  
  chkAuto.onchange = saveSettings;
  chkStartMonitoring.onchange = saveSettings;

  btnStart.onclick = async () => {
    // Convert seconds to milliseconds
    const intervalMs = Math.round(parseFloat(intv.value) * 1000);
    console.log(`Starting monitor with target: ${vol.value}%, interval: ${intervalMs}ms`);
    
    try {
      await invoke("start_monitor", { 
        target: +vol.value, 
        intervalMs: intervalMs, 
        deviceId: devSel.value || null
      });
      console.log("Monitor started successfully");
    } catch (error) {
      console.error("Error starting monitor:", error);
    }
  };
  
  btnStop.onclick = async () => invoke("stop_monitor");
  
  btnExit.onclick = async () => {
    // Uygulamayı tamamen kapat
    await invoke("quit_app");
  };
  
  btnHide.onclick = hideToTray;

  // Load autostart status and setup handler
  try {
    const isEnabled = await invoke("is_autostart_enabled");
    chkAuto.checked = isEnabled;
    
    chkAuto.onchange = async () => { 
      try {
        await invoke("toggle_autostart", { enable: chkAuto.checked });
        await saveSettings();
      } catch (error) {
        console.error("Autostart error:", error);
        chkAuto.checked = !chkAuto.checked; // Revert on error
      }
    };
  } catch (error) { 
    console.warn("Autostart plugin error:", error);
    chkAuto.disabled = true;
    chkAuto.checked = false;
  }

  // Load devices ONCE at startup
  await refreshDevices();
  
  // Auto-start monitoring if enabled
  if (chkStartMonitoring.checked) {
    setTimeout(async () => {
      const intervalMs = Math.round(parseFloat(intv.value) * 1000);
      await invoke("start_monitor", { 
        target: +vol.value, 
        intervalMs: intervalMs, 
        deviceId: devSel.value || null
      });
    }, 1000);
  }
  
  // Show window after everything is loaded - small delay to ensure DOM is ready
  setTimeout(async () => {
    try {
      // Hide loading screen and show main app
      document.getElementById('loading-screen').style.display = 'none';
      document.getElementById('main-app').style.display = 'block';
      
      // Show the window
      await invoke("show_main_window");
      console.log("App initialized and window shown");
    } catch (error) {
      console.warn("Could not show window:", error);
      // Fallback: still show the app even if window show fails
      document.getElementById('loading-screen').style.display = 'none';
      document.getElementById('main-app').style.display = 'block';
    }
  }, 100); // Small delay to ensure smooth transition
}

// Handle window close event for tray notification
window.addEventListener('beforeunload', async (e) => {
  // This will be handled by the Tauri window close event
});

// Disable right-click context menu
document.addEventListener('contextmenu', e => e.preventDefault());

// Disable F12, Ctrl+Shift+I, Ctrl+U, Ctrl+Shift+J
document.addEventListener('keydown', (e) => {
  if (e.key === 'F12' || 
      (e.ctrlKey && e.shiftKey && (e.key === 'I' || e.key === 'J')) ||
      (e.ctrlKey && e.key === 'U')) {
    e.preventDefault();
  }
});

init();
