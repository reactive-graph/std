import * as React from 'react'
import * as ReactDOM from 'react-dom'
import {Voyager} from '@guigiani/graphql-voyager'
import introspectionProvider from './IntrospectionProvider'

import '@guigiani/graphql-voyager/dist/voyager.css'
import '../assets/css/graphql-schema-visualization.css'

const displayOptions = {
  rootType: "Mutation"
}

const workerURI = "../voyager.worker.js"

ReactDOM.render(
  <Voyager introspection = {introspectionProvider} displayOptions = {displayOptions} workerURI = {workerURI} />,
  document.getElementById('voyager'),
)
