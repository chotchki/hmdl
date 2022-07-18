import React, { useState, useEffect } from 'react';
import useAxios from 'axios-hooks';
import { useParams } from 'react-router-dom';

import Accordion from 'react-bootstrap/Accordion';
import Button from 'react-bootstrap/Button';
import Container from 'react-bootstrap/Container';
import Form from 'react-bootstrap/Form';
import InputGroup from 'react-bootstrap/InputGroup';
import ListGroup from 'react-bootstrap/ListGroup';

import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { solid, regular, brands } from '@fortawesome/fontawesome-svg-core/import.macro';

import ClientOfGroup from './ClientOfGroup';

export function ClientGroup(props) {
    let { group } = useParams();
    const [newGroupName, setNewGroupName] = useState("");


    const [{ data, loading, error }, executePut] = useAxios(
        {
            url: '/api/client-groups/' + group,
            method: 'PUT'
        },
        { manual: true }
    );

    const updateGroup = (event) => {
        executePut({
            data: {
                name: newGroupName,
            }
        }).then(event => {
            props.refresh();
        });
    };

    const [{ dataDel, loadingDel, errorDel }, executeDel] = useAxios(
        {
            url: '/api/client-groups/' + group,
            method: 'DELETE'
        },
        { manual: true }
    );

    const deleteGroup = (event) => {
        executeDel().then(event => {
            props.refresh();
        });
    };

    const [{ data: clientGroupDetail, loading: loadingClients, error: errorClients }, executeGet] = useAxios(
        {
            url: '/api/client-groups/' + group,
            method: "GET"
        },
        { manual: true }
    );

    return (
        <Accordion.Item eventKey={group}>
            <Accordion.Header onClick={e => executeGet()}>{group}</Accordion.Header>
            <Accordion.Body>
                <Form>
                    <Form.Group className="mb-3" controlId="groupName">
                        <Form.Label>Group Name</Form.Label>
                        <InputGroup>
                            <Form.Control type="text" defaultValue={group} onChange={event => setNewGroupName(event.target.value)} />
                            <Button variant="primary" onClick={event => updateGroup()}>
                                <FontAwesomeIcon icon={solid('floppy-disk')} />
                            </Button>
                        </InputGroup>
                    </Form.Group>
                </Form>
                <h4>Associated Clients</h4>
                <ListGroup>
                    {clientGroupDetail ? clientGroupDetail.clients.map(client => (
                        <ClientOfGroup key={client.name} client={client} refresh={executeGet} />
                    )) : <ListGroup.Item>No clients</ListGroup.Item>
                    }
                </ListGroup>
                <Container>
                    <Button variant="danger" onClick={event => deleteGroup()}>
                        Delete Group <FontAwesomeIcon icon={solid('trash-can')} />
                    </Button>
                </Container>
            </Accordion.Body>
        </Accordion.Item >
    );
}

export default ClientGroup;