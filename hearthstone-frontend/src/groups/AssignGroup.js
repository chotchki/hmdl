import React, { useState, useEffect } from 'react';

export function AssignGroup(props) {
    if (props.groups == null || props.groups.length === 0) {
        return (
            <Form.Select disabled>
                <option>Setup Groups First</option>
            </Form.Select>
        );
    }
    return (
        <Form>
            <Form.Select>
                <option>Assign Group</option>
                {props.groups.map(group => (
                    <option key={group}>{group}</option>
                ))}
            </Form.Select>
        </Form>
    );
}

export default Domains;