connect-docker : 
	docker compose exec back /bin/bash 

migration-run : 
	diesel migration run

help:
	@echo "Utilisez 'make connect-docker' pour se connecter au conteneur Docker."
	@echo "Utilisez 'make migration-run' pour exécuter les migrations Diesel."

clean:
	@echo "Aucune action de nettoyage spécifiée."