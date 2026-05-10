import { getCurrentWindow } from '@tauri-apps/api/window';
import { windowService } from '../services/windowService';

const appWindow = getCurrentWindow();

export const useFocusSafeDrag = () => {
  const handleMouseDown = async (e: React.MouseEvent) => {
    // Only drag if the primary mouse button is pressed and we're not clicking a button or its children
    if (e.button === 0 && !(e.target as HTMLElement).closest('button')) {
      const startX = e.screenX;
      const startY = e.screenY;
      const initialPos = await appWindow.outerPosition();
      
      const onMouseMove = async (moveEvent: MouseEvent) => {
        const deltaX = moveEvent.screenX - startX;
        const deltaY = moveEvent.screenY - startY;
        
        if (Math.abs(deltaX) > 0 || Math.abs(deltaY) > 0) {
          await windowService.moveNoFocus(
            initialPos.x + deltaX, 
            initialPos.y + deltaY
          );
        }
      };
      
      const onMouseUp = () => {
        window.removeEventListener('mousemove', onMouseMove);
        window.removeEventListener('mouseup', onMouseUp);
      };
      
      window.addEventListener('mousemove', onMouseMove);
      window.addEventListener('mouseup', onMouseUp);
    }
  };

  return { handleMouseDown };
};
