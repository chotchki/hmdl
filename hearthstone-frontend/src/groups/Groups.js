import React, { useState, useEffect } from 'react';
import useAxios from '../utility/useAxios.js';

import Alert from 'react-bootstrap/Alert';
import Button from 'react-bootstrap/Button';
import Spinner from 'react-bootstrap/Spinner';

import AddGroup from './AddGroup.js';
import GroupRow from './GroupRow.js';


export function Groups() {
    const { data, error, loaded } = useAxios("/api/groups", "GET");

    if (error) {
        return (
            <Alert key="danger" variant="danger">
                Error: {error.message}
            </Alert>
        );
    } else if (!loaded) {
        return (
            <Spinner animation="border" role="status">
                <span className="visually-hidden">Loading...</span>
            </Spinner>
        );
    } else {
        return (
            <>
                <table className="table">
                    <thead>
                        <tr>
                            <th scope="col">Name</th>
                            <th scope="col">Model Status</th>
                            <th scope="col">Edit</th>
                            <th scope="col">Remove</th>
                        </tr>
                    </thead>
                    <tbody>
                        {data.map(group => (
                            <GroupRow group={group} />
                        ))}
                    </tbody>
                </table>
                <AddGroup />
            </>
        );
    }
}

export default Groups;