//Code from here: https://aibolik.com/blog/creating-toast-api-with-react-hooks

import React, { useCallback, useContext, useState } from 'react';
import ToastHolder from './ToastHolder.js';

const ToastContext = React.createContext(null);

let id = 1;

const ToastProvider = ({ children }) => {
  const [toasts, setToasts] = useState([]);

  const addToast = useCallback((header, body, background) => {
    setToasts(toasts => [
      ...toasts,
      { id: id++, header, body, background }
    ]);
  }, [setToasts]);

  const removeToast = useCallback(id => {
    setToasts(toasts => toasts.filter(t => t.id !== id));
  }, [setToasts]);

  return (
    <ToastContext.Provider value={{ addToast, removeToast }}>
      <ToastHolder toasts={toasts} />
      {children}
    </ToastContext.Provider>
  );
}

const useToast = () => {
  const toastHelpers = useContext(ToastContext);

  return toastHelpers;
};

export { ToastContext, useToast };
export default ToastProvider;