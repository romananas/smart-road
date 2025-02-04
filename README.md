# Smart Road

### Description

Smart Road est un projet visant à simuler un système de gestion de trafic routier intelligent. L'objectif est d'optimiser la circulation en gérant efficacement les feux de signalisation et la priorité des véhicules en fonction des conditions de circulation.

Le projet est écrit en Rust, un langage sûr et performant, idéal pour la gestion des systèmes en temps réel.

### Fonctionnalités

- Gestion dynamique des feux de circulation

- Priorisation des véhicules selon des critères (urgence, flux de trafic...)

- Simulation d'un réseau routier avec différents types de véhicules

- Optimisation du trafic pour minimiser les embouteillages

- Interface en ligne de commande pour configurer et observer le système

### Installation

#### Prérequis

- Rust (version stable recommandée) : Installation de Rust

#### Cloner le dépôt

 ```bash
 git clone https://github.com/01-edu/public.git
 cd public/subjects/smart-road
 ```

#### Compilation

 ```bash
 cargo build --release
 ```

#### Exécution

``` bash
 cargo run --release
```

### Utilisation

L'application fournit une interface en ligne de commande permettant de lancer et de configurer la simulation. Vous pouvez ajuster différents paramètres comme le nombre de routes, de feux de signalisation, et les priorités des véhicules.

Exemple :

``` bash
 cargo run -- --roads 5 --vehicles 20 --priority emergency
 ```

### Contribuer

Les contributions sont les bienvenues ! Merci de suivre ces étapes pour proposer des modifications :

1. Fork le dépôt

2. Créer une branche pour votre fonctionnalité : git checkout -b nouvelle-feature

3. Commiter vos modifications : git commit -m "Ajout d'une nouvelle fonctionnalité"

4. Pousser vers votre fork : git push origin nouvelle-feature

5. Ouvrir une Pull Request

### Licence

Ce projet est sous licence MIT. Voir le fichier LICENSE pour plus d'informations.