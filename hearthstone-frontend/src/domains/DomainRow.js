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

export function DomainRow(props) {
    const [{ data, error, loading }, executeGetGroups] = useAxios({ url: "/api/groups", method: "GET" });

    //props.domain has the domain info
    //props.groups has the list of all groups
    const [domainName, setDomainName] = useState(props.domain.name);
    const [groupName, setGroupName] = useState("");

    const [{ dataPut, loadingPut, errorPut }, executePut] = useAxios(
        {
            url: '/api/domain/' + props.domain.name,
            method: 'PUT'
        },
        { manual: true }
    );

    const updateDomain = (event) => {
        executePut({
            data: {
                domain: {
                    name: domainName,
                    last_seen: props.domain.last_seen,
                    last_client: props.domain.last_client
                },
                group_name: groupName
            }
        }).then(event => {
            props.refresh();
        });
    };

    const [{ dataDel, loadingDel, errorDel }, executeDel] = useAxios(
        {
            url: '/api/domain/' + props.domain.name,
            method: 'DELETE'
        },
        { manual: true }
    );

    const deleteDomain = (event) => {
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
                <td><Form.Control type="text" defaultValue={domainName} onChange={event => setDomainName(event.target.value)} /></td>
                <td><Timestamp lastSeen={props.domain.last_seen} /></td>
                <td>{props.domain.last_client}</td>
                <td>
                    <InputGroup>
                        <Form.Select onChange={event => setGroupName(event.target.value)}>
                            <option>Assign Group</option>
                            {data.map(group => (
                                <option key={group.name}>{group.name}</option>
                            ))}
                        </Form.Select>
                        <Button variant="primary" onClick={event => updateDomain()}>
                            <FontAwesomeIcon icon={solid('floppy-disk')} />
                        </Button>
                    </InputGroup>
                </td>
                <td>
                    <Button variant="danger" onClick={event => deleteDomain()}>
                        <FontAwesomeIcon icon={solid('trash-can')} />
                    </Button>
                </td>
            </tr>
        );
    }
}

export default DomainRow;