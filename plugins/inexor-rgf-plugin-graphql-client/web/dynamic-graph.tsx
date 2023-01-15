import * as React from 'react'

import 'altair-static/build/dist/styles.css'
import './assets/css/graphql-client.css'
import {AltairWindowOptions, RenderOptions} from 'altair-static'

const endpointURL = window.location.origin + '/dynamic_graph'

const addQuery = `
query {
  add {
    arithmetic_gate {
      lhs
      rhs
      result
    }
  }
}
`

const addQueryWindow: AltairWindowOptions = {
    initialName: 'Add',
    endpointURL,
    initialQuery: addQuery,
}

const renderOptions: RenderOptions = {
    initialName: 'Inexor RGF Dynamic Graph',
    endpointURL,
    preserveState: false,
    initialSettings: {
        'alert.disableUpdateNotification': true,
        'disablePushNotification': true,
        'schema.reloadOnStart': true,
        'plugin.list': [
          'altair-graphql-plugin-graphql-explorer'
        ]
    },
    initialWindows: [
        addQueryWindow
    ]
}

document.addEventListener("DOMContentLoaded", () => {
    // @ts-ignore
    AltairGraphQL.init(renderOptions)
})
