import React from 'react';
import useAxios from 'axios-hooks';

import Alert from 'react-bootstrap/Alert';
import Spinner from 'react-bootstrap/Spinner';

import AddGroup from './AddGroup.js';
import GroupRow from './GroupRow.js';
import Container from 'react-bootstrap/esm/Container.js';


export function Groups() {
    const [{ data, error, loading }, executeGet] = useAxios("/api/groups", "GET");

    if (error) {
        return (
            <Alert key="danger" variant="danger">
                Error: {error.message}
            </Alert>
        );
    } else if (loading) {
        return (
            <Spinner animation="border" role="status">
                <span className="visually-hidden">Loading...</span>
            </Spinner>
        );
    } else {
        return (
            <>
                <Container>
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
                                <GroupRow key={group.name} group={group} refresh={executeGet} />
                            ))}
                        </tbody>
                    </table>
                </Container>
                <Container>
                    <AddGroup refresh={executeGet} />
                </Container>
            </>
        );
    }
}

export default Groups;