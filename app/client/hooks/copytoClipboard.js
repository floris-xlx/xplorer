export const copyToClipboard = (text, triggerNotification, notificationType = 'CopiedToClipboard') => {
  navigator.clipboard
    .writeText(text)
    .then(() => {
      triggerNotification(notificationType);
    })
    .catch((err) => {
      console.error('Failed to copy: ', err);
    });
};
