import './Toast.css';

import React from 'react';
import PropTypes from 'prop-types';
import { useToast } from './ToastProvider';


import Toast from 'react-bootstrap/Toast';
import ToastContainer from 'react-bootstrap/ToastContainer';

export function ToastHolder(props) {
  const { removeToast } = useToast();

  return (
    <ToastContainer position="top-center">
      {props.toasts.map((toast) => (
        <Toast
          key={toast.id}
          id={toast.id}
          bg={toast.status}
          onClose={(e) => removeToast(e.currentTarget.parentElement.parentElement.id)}>
          <Toast.Header><strong className="me-auto">HMDL Says</strong></Toast.Header>
          <Toast.Body>{toast.body}</Toast.Body>
        </Toast>
      ))}
    </ToastContainer>
  );
}

ToastHolder.propTypes = {
  toasts: PropTypes.arrayOf(PropTypes.shape({
    id: PropTypes.number.isRequired,
    body: PropTypes.string.isRequired,
    status: PropTypes.string,
  })),
};

export default ToastHolder;
