import './Toast.css';
import { useEffect } from 'react';
import { useToast } from './ToastProvider';
import Toast from 'react-bootstrap/Toast';
import ToastType from './ToastType';

type ToastComponentProps = {
  toast: ToastType
};

const ToastComponent = ({ toast }: ToastComponentProps): JSX.Element => {
  const { removeToast } = useToast();

  useEffect(() => {
    const timer = setTimeout(() => {
      removeToast(toast.id);
    }, 3000); // delay

    return () => {
      clearTimeout(timer);
    };
  }, [toast.id, removeToast]);

  return (
    <Toast
      id={toast.id}
      bg={toast.status}
      onClose={() => removeToast(toast.id)}>
      <Toast.Header><strong className="me-auto">HMDL Says</strong></Toast.Header>
      <Toast.Body>{toast.body}</Toast.Body>
    </Toast>
  );
};

export default ToastComponent;
