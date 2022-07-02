import React, { useState, useEffect } from 'react';

export function Domains() {
    const [error, setError] = useState(null);
    const [isLoaded, setIsLoaded] = useState(false);
    const [domains, setDomains] = useState([]);

    // Note: the empty deps array [] means
    // this useEffect will run once
    // similar to componentDidMount()
    useEffect(() => {
        fetch("/api/domains")
            .then(res => res.json())
            .then(
                (result) => {
                    setIsLoaded(true);
                    setDomains(result);
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
        return <div>Error: {error.message}</div>;
    } else if (!isLoaded) {
        return <div>Loading...</div>;
    } else {
        return (
            <table className="table">
                <thead>
                    <tr>
                        <th scope="col">Name</th>
                    </tr>
                </thead>
                <tbody>
                    {domains.map(domain => (
                        <tr key={domain.name}>
                            <td>{domain.name}</td>
                        </tr>
                    ))}
                </tbody>
            </table>
        );
    }
}

export default Domains;