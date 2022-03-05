import * as React from 'react'
import * as ReactDOM from 'react-dom'
import {Voyager} from 'graphql-voyager'
import fetch from 'isomorphic-fetch'
import 'graphql-voyager/dist/voyager.css'

function introspectionProvider(query) {
  return fetch(
    window.location.origin + '/graphql',
    {
      method: 'post',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({query: query}),
    }
  ).then(response => response.json())
}

const displayOptions = {
  rootType: "Subscription"
}

ReactDOM.render(
  <Voyager introspection = {introspectionProvider} displayOptions = {displayOptions} />,
  document.getElementById('voyager'),
)
