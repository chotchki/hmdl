import React, { useState, useEffect } from 'react';
import useAxios from 'axios-hooks';
import Form from 'react-bootstrap/Form';
import Button from 'react-bootstrap/Button';

import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { solid, regular, brands } from '@fortawesome/fontawesome-svg-core/import.macro'

export function GroupRow(props) {
    const [editting, setEditting] = useState(false);
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

    if (editting) {
        return (
            <tr>
                <td>
                    <Form.Control type="text" defaultValue={groupName} onChange={event => setGroupName(event.target.value)} />
                </td>
                <td>{modelStatus}</td>
                <td>
                    <Button variant="primary" onClick={event => updateGroup()}>
                        <FontAwesomeIcon icon={solid('floppy-disk')} />
                    </Button>
                    <Button variant="danger" onClick={event => setEditting(false)}>
                        <FontAwesomeIcon icon={solid('xmark')} />
                    </Button>
                </td>
                <td>
                    <Button variant="danger" onClick={event => deleteGroup()}>
                        <FontAwesomeIcon icon={solid('trash-can')} />
                    </Button>
                </td>
            </tr>
        );
    }
    return (
        <tr>
            <td>{groupName}</td>
            <td>{modelStatus}</td>
            <td>
                <Button variant="primary" onClick={event => setEditting(true)}>
                    <FontAwesomeIcon icon={solid('pencil')} />
                </Button>
            </td>
            <td>
                <Button variant="danger" onClick={event => deleteGroup()}>
                    <FontAwesomeIcon icon={solid('trash-can')} />
                </Button>
            </td>
        </tr>
    );
}

export default GroupRow;