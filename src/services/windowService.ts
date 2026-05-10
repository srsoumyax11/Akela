import { invoke } from '@tauri-apps/api/core';

export const windowService = {
  async hide(): Promise<void> {
    await invoke('hide_window');
  },

  async showNoFocus(): Promise<void> {
    await invoke('show_window_no_focus');
  },

  async moveNoFocus(x: number, y: number): Promise<void> {
    await invoke('move_window_no_focus', { x, y });
  }
};
