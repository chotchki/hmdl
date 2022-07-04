import React from 'react';
import useAxios from 'axios-hooks';
import Alert from 'react-bootstrap/Alert';
import Spinner from 'react-bootstrap/Spinner';
import Timestamp from '../utility/Timestamp';

export function Domains() {
    const [{ data, error, loading }] = useAxios("/api/domains", "GET");

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
            <table className="table">
                <thead>
                    <tr>
                        <th scope="col">Name</th>
                        <th scope="col">Last Seen</th>
                        <th scope="col">Last Client</th>
                    </tr>
                </thead>
                <tbody>
                    {data.map(domain => (
                        <tr key={domain.name}>
                            <td>{domain.name}</td>
                            <td><Timestamp lastSeen={domain.last_seen} /></td>
                            <td>{domain.last_client}</td>
                        </tr>
                    ))}
                </tbody>
            </table>
        );
    }
}

export default Domains;