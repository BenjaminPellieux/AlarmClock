# Alarm Clock Application

## Description

L'Alarm Clock Application est une application de réveil développée en Rust, utilisant GTK pour l'interface utilisateur. Elle permet de configurer des alarmes avec des options pour jouer des fichiers WAV ou des stations de radio en ligne à l'heure programmée.

## Fonctionnalités

- **Ajouter une Alarme** : Configurez des alarmes en spécifiant l'heure, les jours de la semaine, et la source audio (fichier WAV ou station de radio).
- **Stations de Radio** : Sélectionnez parmi plusieurs stations de radio populaires.
- **Jouer des Fichiers Audio** : Téléchargez et jouez des fichiers audio depuis YouTube.
- **Interface Utilisateur** : Interface utilisateur interactive et intuitive construite avec GTK.
- **Sauvegarde des Alarmes** : Sauvegardez et chargez les alarmes configurées.

## Prérequis

- Rust
- GTK 3
- yt-dlp
- Navigateur Firefox (pour les cookies)

## Installation

1. **Installer Rust** :
   - Suivez les instructions sur [rust-lang.org](https://www.rust-lang.org/tools/install).

2. **Installer GTK 3** :
   - Suivez les instructions sur [GTK website](https://www.gtk.org/docs/installations/).

3. **Installer yt-dlp** :
   ```sh
   pip install yt-dlp
   ```

4. **Cloner le Répertoire du Projet** :
   ```sh
   git clone https://github.com/votre-utilisateur/alarm-clock-app.git
   cd alarm-clock-app
   ```

5. **Construire le Projet** :
   ```sh
   cargo build
   ```

## Utilisation

1. **Lancer l'Application** :
   ```sh
   cargo run
   ```

2. **Configurer une Alarme** :
   - Cliquez sur "Ajouter un réveil".
   - Remplissez les champs nécessaires : nom de l'alarme, heure, minutes, secondes, et lien vers le fichier audio ou sélectionnez une station de radio.
   - Cliquez sur "Sauvegarder" pour ajouter l'alarme.

3. **Activer/Désactiver une Alarme** :
   - Utilisez les boutons radio pour activer ou désactiver une alarme spécifique.

4. **Supprimer une Alarme** :
   - Cliquez sur le bouton "Supprimer" à côté de l'alarme que vous souhaitez retirer.

## Structure du Projet

- `main.rs` : Point d'entrée de l'application.
- `viewmod.rs` : Gère l'interface utilisateur et les interactions.
- `modelmod.rs` : Définit les structures de données pour les alarmes et les radios.
- `widgetmod.rs` : Définit les widgets GTK utilisés dans l'interface utilisateur.
- `musicmod.rs` : Gère la lecture de musique et de radio.

## Contribuer

Les contributions sont les bienvenues ! Veuillez suivre les étapes ci-dessous pour contribuer :

1. **Fork le Projet**.
2. **Créer une Branche de Fonctionnalité** :
   ```sh
   git checkout -b feature/nouvelle-fonctionnalite
   ```
3. **Commit les Modifications** :
   ```sh
   git commit -m "Ajouter nouvelle fonctionnalité"
   ```
4. **Pousser vers la Branche** :
   ```sh
   git push origin feature/nouvelle-fonctionnalite
   ```
5. **Ouvrir une Pull Request**.

## Licence

Ce projet est sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour plus de détails.

## Remerciements

Merci à tous ceux qui ont contribué à ce projet !

