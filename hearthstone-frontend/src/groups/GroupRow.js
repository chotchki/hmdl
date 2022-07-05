import React, { useState, useEffect } from 'react';
import useAxios from 'axios-hooks';

import Accordion from 'react-bootstrap/Accordion';
import Button from 'react-bootstrap/Button';
import Container from 'react-bootstrap/Container';
import Form from 'react-bootstrap/Form';
import InputGroup from 'react-bootstrap/InputGroup';
import ListGroup from 'react-bootstrap/ListGroup';

import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { solid, regular, brands } from '@fortawesome/fontawesome-svg-core/import.macro';

import DomainOfGroup from './DomainOfGroup';

export function GroupRow(props) {
    const [groupName, setGroupName] = useState(props.group.name);
    const [modelStatus, setModelStatus] = useState(props.group.model_status);

    const [{ data, loading, error }, executePut] = useAxios(
        {
            url: '/api/group/' + props.group.name,
            method: 'PUT'
        },
        { manual: true }
    );

    const updateGroup = (event) => {
        executePut({
            data: {
                name: groupName,
                model_status: modelStatus
            }
        }).then(event => {
            props.refresh();
        });
    };

    const [{ dataDel, loadingDel, errorDel }, executeDel] = useAxios(
        {
            url: '/api/group/' + props.group.name,
            method: 'DELETE'
        },
        { manual: true }
    );

    const deleteGroup = (event) => {
        executeDel().then(event => {
            props.refresh();
        });
    };

    const [{ data: dataDomains, loading: loadingDomains, error: errorDomains }, executeGet] = useAxios(
        {
            url: '/api/group/' + props.group.name + '/domains',
            method: "GET"
        },
        { manual: true }
    );

    return (
        <Accordion.Item eventKey={props.group.name}>
            <Accordion.Header onClick={e => executeGet()}>{groupName}</Accordion.Header>
            <Accordion.Body>
                <Form>
                    <Form.Group className="mb-3" controlId="groupName">
                        <Form.Label>Group Name</Form.Label>
                        <InputGroup>
                            <Form.Control type="text" defaultValue={groupName} onChange={event => setGroupName(event.target.value)} />
                            <Button variant="primary" onClick={event => updateGroup()}>
                                <FontAwesomeIcon icon={solid('floppy-disk')} />
                            </Button>
                        </InputGroup>
                    </Form.Group>
                </Form>
                <h4>Associated Domains</h4>
                <ListGroup>
                    {dataDomains ? dataDomains.map(domain => (
                        <DomainOfGroup key={domain.name} domain={domain.name} group={groupName} refresh={executeGet} />
                    )) : <ListGroup.Item>No domains</ListGroup.Item>
                    }
                </ListGroup>
                <Container>
                    <Form>
                        <Button variant="danger" onClick={event => deleteGroup()}>
                            Delete Group <FontAwesomeIcon icon={solid('trash-can')} />
                        </Button>
                    </Form>
                </Container>
            </Accordion.Body>
        </Accordion.Item >
    );
}

export default GroupRow;