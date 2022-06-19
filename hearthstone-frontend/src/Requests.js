import React from 'react';

class Requests extends React.Component {
    render() {
        return <table class="table">
            <thead>
                <tr>
                    <th scope="col">Record Type</th>
                    <th scope="col">Name</th>
                    <th scope="col">Client</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>A</td>
                    <td>google.com</td>
                    <td>10.0.0.0</td>
                </tr>
            </tbody>
        </table>;
    }
}

export default Requests;