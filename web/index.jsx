import * as React from 'react'

import 'altair-static/build/dist/styles.css'

import { renderAltair } from 'altair-static'

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

// document.getElementById('altair').innerHTML = renderAltair(renderOptions)
//
// window.AltairGraphQL.init({
//     baseURL: '/flow-editor/altair',
//     endpointURL: '/graphql',
//     initialQuery: `
// query {
//   instances {
//     status: entities(id: "d5f6e205-ccc6-4b63-9675-014831ecbf3f") {
//       id,
//       type {name},
//       properties
//     }
//   }
// }`
// });
