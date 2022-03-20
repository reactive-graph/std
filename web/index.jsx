import * as React from 'react'

import 'altair-static/build/dist/styles.css'

const initialQuery = `
query {
  types {
    entities {
      name
      description
      components {
        name
      }
      properties {
        name
        dataType
        socketType
        extensions {
          name
        }
      }
      extensions {
        name
      }
    }
  }
}
`

const renderOptions = {
    baseURL: '/graphql-client',
    endpointURL: '/graphql',
    // subscriptionsEndpoint: 'ws://localhost:4000/subscriptions',
    initialQuery,
}

window.AltairGraphQL.init(renderOptions)
