import React, { useState, useEffect } from 'react';
import useAxios from 'axios-hooks';

import Button from 'react-bootstrap/Button';
import Container from "react-bootstrap/Container";
import ListGroup from 'react-bootstrap/ListGroup';
import Stack from 'react-bootstrap/Stack';

import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { solid } from '@fortawesome/fontawesome-svg-core/import.macro'


export function ClientOfGroup(props) {
    const [{ data, loading, error }, executeDel] = useAxios(
        {
            url: '/api/clients/' + props.client.name + '/group',
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
            <Container fluid>
                <Stack direction="horizontal" gap={3}>
                    <span className="me-auto">{props.client.name} - {props.client.ip} - {props.client.mac}</span>
                    <Button variant="danger" onClick={event => deleteClientGroup()}>
                        <FontAwesomeIcon icon={solid('trash-can')} />
                    </Button>
                </Stack>
            </Container>
        </ListGroup.Item>
    );
}

export default ClientOfGroup;