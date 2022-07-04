import React, { useState, useEffect } from 'react';
import Button from 'react-bootstrap/Button';

import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { solid, regular, brands } from '@fortawesome/fontawesome-svg-core/import.macro'

export function GroupRow(props) {
    const [editting, setEditting] = useState(false);

    return (
        <tr key={props.group.name}>
            <td>{props.group.name}</td>
            <td></td>
            <td>
                <Button variant="primary">
                    <FontAwesomeIcon icon={solid('pencil')} />
                </Button>
            </td>
            <td>
                <Button variant="danger">
                    <FontAwesomeIcon icon={solid('trash-can')} />
                </Button>
            </td>
        </tr>
    );
}

export default GroupRow;