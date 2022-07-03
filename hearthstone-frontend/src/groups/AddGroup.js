import React, { useState, useEffect } from 'react';
import useAxios from '../utility/useAxios';
import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form';

import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { solid, regular, brands } from '@fortawesome/fontawesome-svg-core/import.macro'

export function AddGroup(props) {
    const { data, error, loaded } = useAxios(
        "/api/group/", "POST",
        {
            message: "Hello World",
        }
    );
    const submitGroup = (event) => {

    };

    return (
        <Form onSubmit={addGroup}>
            <Form.Group className="mb-3" controlId="name">
                <Form.Label>Group Name</Form.Label>
                <Form.Control type="text" placeholder="Enter group name" />
            </Form.Group>
            <Button variant="primary" type="submit">
                Add <FontAwesomeIcon icon={solid('plus')} />
            </Button>
        </Form>
    );
}

export default AddGroup;