// Code from here: https://aibolik.com/blog/creating-toast-api-with-react-hooks

import { createContext, useCallback, useContext, useState } from 'react';
import PropTypes from 'prop-types';
import ToastHolder from './ToastHolder';
import ToastType from './ToastType';
import { AxiosError } from 'axios';
import ToastStatus from './ToastStatus';

let id = 1;

interface ToastContextProps {
  toasts: Array<ToastType>;
  addToastAxiosError: (error: AxiosError, body: string) => void;
  addToastSuccess: (body: string) => void;
  removeToast: (id_str: string) => void;
}

const ToastContext = createContext<ToastContextProps>({
  toasts: new Array<ToastType>(),
  addToastAxiosError: (error: AxiosError, body: string) => { console.log('ToastContext wrong'); },
  addToastSuccess: (body: string) => { console.log('ToastContext wrong'); },
  removeToast: (id_str: string) => { console.log('ToastContext wrong'); }
});

type ToastProviderProps = {
  children: JSX.Element
};

const ToastProvider = ({ children }: ToastProviderProps) => {
  const [toasts, setToasts] = useState(Array<ToastType>);

  const addToastAxiosError = useCallback((error: AxiosError, body: string) => {
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

    setToasts((toasts: Array<ToastType>) => [
      ...toasts,
      { id: 'toast-' + id++, body, status: ToastStatus.Error },
    ]);
  }, []);

  const addToastSuccess = useCallback((body: string) => {
    setToasts((toasts: Array<ToastType>) => [
      ...toasts,
      { id: 'toast-' + id++, body, status: ToastStatus.Ok },
    ]);
  }, []);

  const removeToast = useCallback((id_str: string) => {
    setToasts((toasts: Array<ToastType>) => {
      return toasts.filter((t) => 'toast-' + t.id !== id_str);
    });
  }, []);

  return (
    <ToastContext.Provider value={{ toasts, addToastAxiosError, addToastSuccess, removeToast }}>
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
