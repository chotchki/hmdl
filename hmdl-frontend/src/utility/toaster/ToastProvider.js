// Code from here: https://aibolik.com/blog/creating-toast-api-with-react-hooks

import React, { useCallback, useContext, useState } from 'react';
import PropTypes from 'prop-types';
import ToastHolder from './ToastHolder.js';

const ToastContext = React.createContext(null);

let id = 1;

const ToastProvider = ({ children }) => {
  const [toasts, setToasts] = useState([]);

  const addToastAxiosError = useCallback((error, body) => {
    // Handling Error from https://axios-http.com/docs/handling_errors
    if (error.response) {
      // The request was made and the server responded with a status code
      // that falls out of the range of 2xx
      console.log(error.response.data);
      console.log(error.response.status);
      console.log(error.response.headers);
    } else if (error.request) {
      // The request was made but no response was received
      // `error.request` is an instance of XMLHttpRequest in the browser and an instance of
      // http.ClientRequest in node.js
      console.log(error.request);
    } else {
      // Something happened in setting up the request that triggered an Error
      console.log('Error', error.message);
    }
    console.log(error.config);

    body = body + ' Check the console for additonal information.';

    setToasts((toasts) => [
      ...toasts,
      { id: 'toast-' + id++, body, status: 'danger' },
    ]);
  }, []);

  const addToastSuccess = useCallback((body) => {
    setToasts((toasts) => [
      ...toasts,
      { id: 'toast-' + id++, body, status: 'success' },
    ]);
  }, []);

  const removeToast = useCallback((id) => {
    setToasts((toasts) => {
      return toasts.filter((t) => t.id !== id);
    });
  }, []);

  return (
    <ToastContext.Provider value={{ addToastAxiosError, addToastSuccess, removeToast }}>
      <ToastHolder toasts={toasts} />
      {children}
    </ToastContext.Provider>
  );
};

ToastProvider.propTypes = {
  children: PropTypes.element,
};

const useToast = () => {
  const toastHelpers = useContext(ToastContext);

  return toastHelpers;
};

export { ToastContext, useToast };
export default ToastProvider;
