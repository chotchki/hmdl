import React, { useState, useEffect } from 'react';
import useAxios from 'axios-hooks';

import Alert from 'react-bootstrap/Alert';
import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form';
import InputGroup from 'react-bootstrap/InputGroup';
import Spinner from 'react-bootstrap/Spinner';

import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { solid, regular, brands } from '@fortawesome/fontawesome-svg-core/import.macro'

import Timestamp from '../utility/Timestamp';

export function ClientRow(props) {
    const [{ data, error, loading }, executeGetGroups] = useAxios({ url: "/api/client-groups", method: "GET" });
    const [clientName, setClientName] = useState(props.client.name);
    const [groupName, setGroupName] = useState("");

    const [{ dataPut, loadingPut, errorPut }, executePut] = useAxios(
        {
            url: '/api/clients/' + props.client.name,
            method: 'PUT'
        },
        { manual: true }
    );

    const updateClient = (event) => {
        executePut({
            data: {
                client: {
                    name: clientName,
                    ip: props.client.ip,
                    mac: props.client.mac
                },
                group_name: groupName
            }
        }).then(event => {
            props.refresh();
        });
    };

    const [{ dataDel, loadingDel, errorDel }, executeDel] = useAxios(
        {
            url: '/api/clients/' + clientName,
            method: 'DELETE'
        },
        { manual: true }
    );

    const deleteClient = (event) => {
        executeDel().then(event => {
            props.refresh();
        });
    };

    if (error) {
        return (
            <tr>
                <td>
                    <Alert key="danger" variant="danger">
                        Error: {error.message}
                    </Alert>
                </td>
            </tr>
        );
    } else if (loading) {
        return (
            <tr>
                <td>
                    <Spinner animation="border" role="status">
                        <span className="visually-hidden">Loading...</span>
                    </Spinner>
                </td>
            </tr>
        );
    } else {
        return (
            <tr>
                <td><Form.Control type="text" defaultValue={clientName} onChange={event => setClientName(event.target.value)} /></td>
                <td>{props.client.ip}</td>
                <td>{props.client.mac}</td>
                <td>
                    <InputGroup>
                        <Form.Select onChange={event => setGroupName(event.target.value)}>
                            <option>Assign Group</option>
                            {data.map(group => (
                                <option key={group}>{group}</option>
                            ))}
                        </Form.Select>
                        <Button variant="primary" onClick={event => updateClient()}>
                            <FontAwesomeIcon icon={solid('floppy-disk')} />
                        </Button>
                    </InputGroup>
                </td>
                <td>
                    <Button variant="danger" onClick={event => deleteClient()}>
                        <FontAwesomeIcon icon={solid('trash-can')} />
                    </Button>
                </td>
            </tr>
        );
    }
}

export default ClientRow;