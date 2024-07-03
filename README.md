# Flux Atlas

A comprehensive map of your cluster's Flux state.

## About

Flux Atlas is a simple web interface for monitoring the state of Kubernetes clusters running [Flux CD](https://fluxcd.io).

## Installation

First, ensure Flux CD is [installed](https://fluxcd.io/flux/installation/) on the target cluster then simply apply the resource in `deploy/flux.yml` to your cluster with `kubectl`.

## Contributing

Once you've created a project and installed dependencies with `npm install` start a development server:

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

The development server will pick up your local `kubectl` context.

### Testing

To test your changes, use `npm run test` - this will include both unit tests and integration tests (via playwright).

### Tilt

A [tilt](https://tilt.dev) file is provided in the repo root to simplify the process of iteratively testing your changes on a live cluster.
