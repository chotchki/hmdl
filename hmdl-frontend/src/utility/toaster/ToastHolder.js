import './Toast.css';

import React, { useState } from 'react';
import { useToast } from './ToastProvider';

import Toast from 'react-bootstrap/Toast';
import ToastContainer from 'react-bootstrap/ToastContainer'

export function ToastHolder(props) {
    const { removeToast } = useToast();

    return (
        <ToastContainer position="top-center">
            {props.toasts && props.toasts.length > 0 ? props.toasts.map(toast => (
                <Toast key={toast.id} id={toast.id} bg={toast.background ? toast.background : 'success'} onClose={e => removeToast(e.currentTarget.parentElement.parentElement.id)}>
                    {toast.header ? <Toast.Header><strong className="me-auto">{toast.header}</strong></Toast.Header> : ''}
                    {toast.body ? <Toast.Body>{toast.body}</Toast.Body> : ''}
                </Toast>
            )) : ''
            }
        </ToastContainer>
    );
}

export default ToastHolder;