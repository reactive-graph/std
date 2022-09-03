import * as React from 'react'
import * as ReactDOM from 'react-dom'
import {Voyager} from 'graphql-voyager'
import introspectionProvider from './IntrospectionProvider'

import 'graphql-voyager/dist/voyager.css'
import '../assets/css/graphql-schema-visualization.css'

const displayOptions = {
  rootType: "Query",
  hideSettings: true
}

const workerURI = "../voyager.worker.js"

ReactDOM.render(
  <Voyager introspection = {introspectionProvider} displayOptions = {displayOptions} workerURI = {workerURI} />,
  document.getElementById('voyager'),
)
