import React, { useState, useEffect } from 'react';
import Alert from 'react-bootstrap/Alert';
import Spinner from 'react-bootstrap/Spinner';

export function Groups() {
    const [error, setError] = useState(null);
    const [isLoaded, setIsLoaded] = useState(false);
    const [groups, setGroups] = useState([]);

    useEffect(() => {
        fetch("/api/groups")
            .then(res => res.json())
            .then(
                (result) => {
                    setIsLoaded(true);
                    setGroups(result);
                },
                // Note: it's important to handle errors here
                // instead of a catch() block so that we don't swallow
                // exceptions from actual bugs in components.
                (error) => {
                    setIsLoaded(true);
                    setError(error);
                }
            )
    }, [])

    if (error) {
        return (
            <Alert key="danger" variant="danger">
                Error: {error.message}
            </Alert>
        );
    } else if (!isLoaded) {
        return (
            <Spinner animation="border" role="status">
                <span className="visually-hidden">Loading...</span>
            </Spinner>
        );
    } else {
        return (
            <>
                <span>Groups</span>
            </>
        );
    }
}

export default Groups;