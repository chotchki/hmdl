import './Toast.css';

import ToastContainer from 'react-bootstrap/ToastContainer';
import ToastComponent from './ToastComponent';
import ToastType from './ToastType';

type ToastHolderProps = {
  toasts: Array<ToastType>
};

const ToastHolder = ({ toasts }: ToastHolderProps): JSX.Element => {
  return (
    <ToastContainer position="top-center">
      {toasts.map((toast) => (
        <ToastComponent key={toast.id} toast={toast} />
      ))}
    </ToastContainer>
  );
};

export default ToastHolder;
