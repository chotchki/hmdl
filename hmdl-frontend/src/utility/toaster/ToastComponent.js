import './Toast.css';

import React, { useEffect } from 'react';
import PropTypes from 'prop-types';
import { useToast } from './ToastProvider';
import Toast from 'react-bootstrap/Toast';

export function ToastComponent(props) {
  const { removeToast } = useToast();

  useEffect(() => {
    const timer = setTimeout(() => {
      removeToast(props.toast.id);
    }, 3000); // delay

    return () => {
      clearTimeout(timer);
    };
  }, [props.toast.id, removeToast]);

  return (
    <Toast
      id={props.toast.id}
      bg={props.toast.status}
      onClose={() => removeToast(props.toast.id)}>
      <Toast.Header><strong className="me-auto">HMDL Says</strong></Toast.Header>
      <Toast.Body>{props.toast.body}</Toast.Body>
    </Toast>
  );
}

ToastComponent.propTypes = {
  toast: PropTypes.shape({
    id: PropTypes.string.isRequired,
    body: PropTypes.string.isRequired,
    status: PropTypes.string.isRequired,
  }),
};

export default ToastComponent;
