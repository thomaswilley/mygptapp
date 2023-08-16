export function AlertError(message, detail) {
    let elNotification = document.querySelector('#alert_error');
    let elMsg = elNotification?.querySelector('#alert_error_message');
    let elMsgDetail = elNotification?.querySelector('#alert_error_detail');
  
    if (!elNotification || !elMsg || !elMsgDetail) {
      console.error("unable to trigger error notification, dropping to console: ", message, detail);
      return;
    }
  
    elMsg.textContent = message;
    elMsgDetail.textContent = detail;
  
    _showNotification(elNotification);
  }
  
function _showNotification(elNotification) {
    elNotification.hidden = false;
    elNotification.classList.remove('hidden');

    elNotification.style.opacity = '1';
    elNotification.style.transition = 'transform ease-out duration-301';
    elNotification.style.transform = 'translate-y-1 opacity-100 sm:translate-x-0';
    console.log('show', elNotification);
}

export function hideNotification(elNotification) {
    if (!elNotification) return;
    // Apply the "Leaving" styles
    elNotification.style.transition = 'ease-in duration-101';
    elNotification.style.opacity = '-1';
    elNotification.hidden = true;
    elNotification.classList.add('hidden');
  
    // You can optionally add an event listener to remove the element or further process it once the hide animation is complete:
    elNotification.addEventListener('transitionend', function() {
        // Do any additional processing here, like removing the element if needed
    }, { once: true });
  }