import './Toast.css';

import React from 'react';
import PropTypes from 'prop-types';

import ToastContainer from 'react-bootstrap/ToastContainer';

import ToastComponent from './ToastComponent.js';

export function ToastHolder(props) {
  return (
    <ToastContainer position="top-center">
      {props.toasts.map((toast) => (
        <ToastComponent key={toast.id} toast={toast} />
      ))}
    </ToastContainer>
  );
}

ToastHolder.propTypes = {
  toasts: PropTypes.arrayOf(PropTypes.shape({
    id: PropTypes.string.isRequired,
    body: PropTypes.string.isRequired,
    status: PropTypes.string,
  })),
};

export default ToastHolder;
