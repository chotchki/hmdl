import React, { useState, useEffect } from 'react';
import useAxios from 'axios-hooks';

import Alert from 'react-bootstrap/Alert';
import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form';
import InputGroup from 'react-bootstrap/InputGroup';
import ListGroup from 'react-bootstrap/ListGroup';
import Spinner from 'react-bootstrap/Spinner';

import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { solid, regular, brands } from '@fortawesome/fontawesome-svg-core/import.macro'

import Timestamp from '../utility/Timestamp';
import Col from 'react-bootstrap/esm/Col';

export function ClientOfGroup(props) {
    const [{ data, loading, error }, executeDel] = useAxios(
        {
            url: '/api/clients/' + props.client + '/group',
            method: 'DELETE'
        },
        { manual: true }
    );

    const deleteClientGroup = (event) => {
        executeDel().then(event => {
            props.refresh();
        });
    };

    return (
        <ListGroup.Item>
            <span>{props.client.name} - {props.client.ip} - {props.client.mac}</span>
            <Button variant="danger" onClick={event => deleteClientGroup()}>
                <FontAwesomeIcon icon={solid('trash-can')} />
            </Button>


        </ListGroup.Item>
    );
}

export default ClientOfGroup;