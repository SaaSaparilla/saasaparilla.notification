* include a thin, pull- and resolve-only container registry client to mount the host `/run/containerd/containerd.sock`
  in order to avoid having to push images to `localhost:5000` after building them (and to use the host's image cache).
  This could be https://github.com/mcronce/oci-registry/tree/master with some modifications.
* alternately, examine [configuration options](https://distribution.github.io/distribution/about/configuration/) for
  the canonical registry [implementation](https://hub.docker.com/_/registry) to see if there is another way to 
  accomplish the objective of automagically serving images that are present on the host into the kind cluster.  
  [This example config](https://github.com/distribution/distribution-library-image/blob/master/config-example.yml) may 
  or may not be helpful.
* replace nginx with brixt (gateway/ingress controller)
* crossplane or tekton? (job runner/container builder)
* something to trigger a rebuild whenever the gitrepository is updated (does flux sourcecontroller emit events?)
