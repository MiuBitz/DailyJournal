function getInvoke() {
  if (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke) {
    return window.__TAURI__.core.invoke;
  }
  if (window.__TAURI__ && window.__TAURI__.invoke) {
    return window.__TAURI__.invoke;
  }
  return async (cmd, args) => {
    console.error('Tauri API not ready or window.__TAURI__ missing when calling command:', cmd, args);
  };
}

function getListen() {
  if (window.__TAURI__ && window.__TAURI__.event && window.__TAURI__.event.listen) {
    return window.__TAURI__.event.listen;
  }
  return async () => {};
}

let currentConfig = null;

function showToast(message) {
  const toast = document.getElementById('toast');
  toast.textContent = message;
  toast.classList.remove('hidden');
  setTimeout(() => {
    toast.classList.add('hidden');
  }, 4000);
}

async function loadConfig() {
  try {
    const invoke = getInvoke();
    currentConfig = await invoke('get_config');
    console.log('Loaded AppConfig:', currentConfig);

    if (!currentConfig) {
      console.warn('AppConfig returned null or undefined');
      return;
    }

    // Set today date
    const todayStr = await invoke('get_today_str');
    document.getElementById('today-heading').textContent = `📅 Today — ${todayStr}`;

    // Fill storage folder
    document.getElementById('folder-path').value = currentConfig.output_folder || '';

    // Fill autostart
    document.getElementById('chk-autostart').checked = !!currentConfig.start_with_windows;

    // Fill reminders
    if (currentConfig.reminders) {
      document.getElementById('chk-rem-morning').checked = !!currentConfig.reminders.morning;
      document.getElementById('chk-rem-afternoon').checked = !!currentConfig.reminders.afternoon;
      document.getElementById('chk-rem-evening').checked = !!currentConfig.reminders.evening;

      document.getElementById('time-input-morning').value = currentConfig.reminders.morning_time || '12:00';
      document.getElementById('time-input-afternoon').value = currentConfig.reminders.afternoon_time || '18:00';
      document.getElementById('time-input-evening').value = currentConfig.reminders.evening_time || '22:00';
    }

    updateTimeWindows();
  } catch (err) {
    console.error('Failed to load config:', err);
  }
}

function updateTimeWindows() {
  if (!currentConfig || !currentConfig.reminders) return;
  const rem = currentConfig.reminders;
  document.getElementById('time-morning').textContent = `Time Window: 05:00 – ${rem.morning_time}`;
  document.getElementById('time-afternoon').textContent = `Time Window: ${rem.morning_time} – ${rem.afternoon_time}`;
  document.getElementById('time-evening').textContent = `Time Window: ${rem.afternoon_time} – ${rem.evening_time}`;
}

async function saveCurrentConfig() {
  if (!currentConfig) return;
  try {
    const invoke = getInvoke();
    await invoke('save_config', { config: currentConfig });
  } catch (err) {
    console.error('Failed to save config:', err);
  }
}

// Event Listeners
document.getElementById('btn-write-morning').addEventListener('click', async () => {
  const invoke = getInvoke();
  await invoke('open_journal_file', { block: 'morning' });
});
document.getElementById('btn-write-afternoon').addEventListener('click', async () => {
  const invoke = getInvoke();
  await invoke('open_journal_file', { block: 'afternoon' });
});
document.getElementById('btn-write-evening').addEventListener('click', async () => {
  const invoke = getInvoke();
  await invoke('open_journal_file', { block: 'evening' });
});

document.getElementById('btn-create-today').addEventListener('click', async () => {
  try {
    const invoke = getInvoke();
    await invoke('create_today_files');
    showToast("Today's journal files are ready.");
  } catch (err) {
    showToast(String(err));
  }
});

document.getElementById('btn-open-folder').addEventListener('click', async () => {
  const invoke = getInvoke();
  await invoke('open_folder');
});

document.getElementById('btn-browse-folder').addEventListener('click', async () => {
  try {
    const invoke = getInvoke();
    const newFolder = await invoke('pick_folder');
    if (newFolder) {
      currentConfig.output_folder = newFolder;
      document.getElementById('folder-path').value = newFolder;
      await saveCurrentConfig();
    }
  } catch (err) {
    console.error(err);
  }
});

document.getElementById('chk-autostart').addEventListener('change', async (e) => {
  if (!currentConfig) return;
  const enabled = e.target.checked;
  currentConfig.start_with_windows = enabled;
  try {
    const invoke = getInvoke();
    await invoke('set_autostart', { enabled });
    await saveCurrentConfig();
  } catch (err) {
    showToast(String(err));
  }
});

document.getElementById('chk-rem-morning').addEventListener('change', async (e) => {
  if (!currentConfig || !currentConfig.reminders) return;
  currentConfig.reminders.morning = e.target.checked;
  await saveCurrentConfig();
});
document.getElementById('chk-rem-afternoon').addEventListener('change', async (e) => {
  if (!currentConfig || !currentConfig.reminders) return;
  currentConfig.reminders.afternoon = e.target.checked;
  await saveCurrentConfig();
});
document.getElementById('chk-rem-evening').addEventListener('change', async (e) => {
  if (!currentConfig || !currentConfig.reminders) return;
  currentConfig.reminders.evening = e.target.checked;
  await saveCurrentConfig();
});

document.getElementById('time-input-morning').addEventListener('change', async (e) => {
  if (!currentConfig || !currentConfig.reminders) return;
  currentConfig.reminders.morning_time = e.target.value;
  updateTimeWindows();
  await saveCurrentConfig();
});
document.getElementById('time-input-afternoon').addEventListener('change', async (e) => {
  if (!currentConfig || !currentConfig.reminders) return;
  currentConfig.reminders.afternoon_time = e.target.value;
  updateTimeWindows();
  await saveCurrentConfig();
});
document.getElementById('time-input-evening').addEventListener('change', async (e) => {
  if (!currentConfig || !currentConfig.reminders) return;
  currentConfig.reminders.evening_time = e.target.value;
  updateTimeWindows();
  await saveCurrentConfig();
});

// Settings Modal Events
document.getElementById('btn-settings-toggle').addEventListener('click', () => {
  document.getElementById('settings-modal').classList.remove('hidden');
});
document.getElementById('btn-close-settings').addEventListener('click', () => {
  document.getElementById('settings-modal').classList.add('hidden');
});
document.getElementById('btn-save-settings').addEventListener('click', () => {
  document.getElementById('settings-modal').classList.add('hidden');
});

// Modal Events
let currentReminderBlock = null;

const listen = getListen();
listen('reminder-triggered', (event) => {
  currentReminderBlock = event.payload;
  document.getElementById('modal-title').textContent = `${currentReminderBlock.toUpperCase()} Check-in Reminder`;
  document.getElementById('modal-desc').textContent = `Take a minute to record your ${currentReminderBlock} journal entry.`;
  document.getElementById('reminder-modal').classList.remove('hidden');
});

document.getElementById('btn-modal-write').addEventListener('click', async () => {
  if (currentReminderBlock) {
    const invoke = getInvoke();
    await invoke('open_journal_file', { block: currentReminderBlock });
  }
  document.getElementById('reminder-modal').classList.add('hidden');
});

document.getElementById('btn-modal-later').addEventListener('click', () => {
  document.getElementById('reminder-modal').classList.add('hidden');
});

// Init
window.addEventListener('DOMContentLoaded', () => {
  setTimeout(loadConfig, 100);
});
