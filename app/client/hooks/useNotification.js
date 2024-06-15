import { useState, useEffect } from 'react';

const useNotification = (initialState = null) => {
  const [notification, setNotification] = useState(initialState);
  const [isVisible, setIsVisible] = useState(false);
  const [shouldAnimate, setShouldAnimate] = useState(false);

  useEffect(() => {
    if (notification) {
      setIsVisible(true);
      setShouldAnimate(true);

      const resetNotificationTimer = setTimeout(() => {
        setNotification(null);
      }, 5500);

      return () => clearTimeout(resetNotificationTimer);
    }
  }, [notification]);

  useEffect(() => {
    if (shouldAnimate) {
      const timer = setTimeout(() => {
        setIsVisible(false);
        setShouldAnimate(false);
      }, 5500);

      return () => clearTimeout(timer);
    }
  }, [shouldAnimate]);

  const triggerNotification = (messageType) => {
    setNotification(messageType);
  };

  return {
    notification,
    isVisible,
    shouldAnimate,
    triggerNotification,
  };
};

export default useNotification;
