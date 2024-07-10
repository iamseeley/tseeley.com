export const applyGlow = () => {
    const links = document.querySelectorAll('a:not(nav a)');
  
    links.forEach(link => {
      const glowPoint = document.createElement('div');
      glowPoint.classList.add('glow-point');
      link.appendChild(glowPoint);
  
      link.addEventListener('mouseenter', () => {
        glowPoint.style.opacity = '1';
        glowPoint.style.transform = 'translate(-50%, -50%) scale(1)';
        glowPoint.style.filter = 'blur(15px)';
      });
  
      link.addEventListener('mouseleave', () => {
        glowPoint.style.opacity = '0';
        glowPoint.style.transform = 'translate(-50%, -50%) scale(0.1)';
        glowPoint.style.filter = 'blur(20px)';
      });
    });
  };
  
  