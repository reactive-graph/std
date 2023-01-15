import * as React from 'react'

import 'altair-static/build/dist/styles.css'
import './assets/css/graphql-client.css'
import {AltairWindowOptions, RenderOptions} from 'altair-static'

const endpointURL = window.location.origin + '/graphql'

const entityTypesQuery = `
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

const entityInstancesQuery = `
query {
  instances {
    entities {
      id
      label
    }
  }
}
`

const entityTypesQueryWindow: AltairWindowOptions = {
    initialName: 'Entity Types',
    endpointURL,
    subscriptionsEndpoint: endpointURL,
    initialQuery: entityTypesQuery,
}

const entityInstancesQueryWindow: AltairWindowOptions = {
    initialName: 'Entity Instances',
    endpointURL,
    subscriptionsEndpoint: endpointURL,
    initialQuery: entityInstancesQuery
}

const renderOptions: RenderOptions = {
    initialName: 'Inexor RGF Graph',
    endpointURL,
    subscriptionsEndpoint: endpointURL,
    preserveState: false,
    initialSettings: {
        'alert.disableUpdateNotification': true,
        'disablePushNotification': true,
        'schema.reloadOnStart': true,
        'plugin.list': [
          'altair-graphql-plugin-graphql-explorer'
        ]
    },
    initialQuery: entityTypesQuery,
    initialWindows: [
        entityTypesQueryWindow,
        entityInstancesQueryWindow
    ]
}

document.addEventListener("DOMContentLoaded", () => {
    // @ts-ignore
    AltairGraphQL.init(renderOptions)
})
