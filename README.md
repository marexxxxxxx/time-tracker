---

## 🏗 Installation & Nutzung

Da die App aktuell auf statischem HTML und Tailwind via CDN basiert, ist keine komplexe Installation nötig:

1.  **Repository klonen:**
    ```bash
    git clone [https://github.com/DEIN-BENUTZERNAME/screen-time-app.git](https://github.com/DEIN-BENUTZERNAME/screen-time-app.git)
    ```
2.  **Dateien öffnen:**
    Öffne die `index.html` einfach in einem modernen Webbrowser deiner Wahl.

> **Hinweis:** Da Tailwind CSS und die Icons über CDNs geladen werden, ist eine aktive Internetverbindung für die korrekte Darstellung erforderlich.

---

## 📈 Geplante Erweiterungen

*   [ ] **Local Storage Integration:** Speichern von eingestellten Limits im Browser.
*   [ ] **Dark Mode Toggle:** Dynamischer Wechsel zwischen Light- und Dark-Theme.
*   [ ] **Interaktive Charts:** Einbindung von Chart.js für echte Datenvisualisierung.
*   [ ] **Backend-Anbindung:** Eine API zur Synchronisierung der echten Nutzungsdaten.

---

## 📄 Lizenz

Dieses Projekt ist unter der MIT-Lizenz lizenziert. Weitere Details findest du in der [LICENSE](LICENSE) Datei.

---

Soll ich dir noch helfen, die Verknüpfungen zwischen den Seiten im Code direkt anzupassen, damit dieHier ist ein Entwurf für eine professionelle und übersichtliche `README.md`, die perfekt zu dem modernen Design deiner App passt.

---

# 📊 ScreenTime Dashboard

Ein modernes, minimalistisches Interface zur Analyse der Bildschirmzeit und Steigerung der persönlichen Produktivität. Dieses Projekt bietet ein sauberes Dashboard-Erlebnis, um Nutzungsmuster zu visualisieren, Fokus-Sitzungen zu tracken und App-Limits zu verwalten.

## 🚀 Features

Das Projekt besteht aus drei Kernkomponenten, die eine nahtlose Nutzererfahrung bieten:

*   **Übersichts-Dashboard:** Eine visuelle Zusammenfassung der täglichen Nutzung mit Barcharts und Kategorien-Analyse.
*   **Productivity Tracker:** Detaillierte Einsichten in "Deep Work"-Sessions und der Vergleich zwischen produktiver Zeit und Freizeit.
*   **App-Blocker & Limits:** Verwaltung von Einschränkungen für soziale Medien und Unterhaltungs-Apps inklusive Status-Toggles.

---

## 🛠 Tech Stack

Dieses Projekt nutzt moderne Web-Technologien für ein responsives und performantes UI:

*   **HTML5:** Strukturierte semantische Inhalte.
*   **Tailwind CSS:** Utility-first CSS-Framework für das Styling (via CDN eingebunden).
*   **Google Fonts & Material Symbols:** Nutzung der "Inter"-Schriftart und "Material Symbols Outlined" für ein konsistentes Icon-Design.
*   **Responsive Design:** Optimiert für Desktop und mobile Ansichten.

---

## 📂 Dateistruktur

Damit die Navigation innerhalb der App funktioniert, sollte die Struktur wie folgt aussehen:
```text
/
├── index.html          # Das Haupt-Dashboard (Overview)
├── productivity.html   # Der Productivity Tracker
└── blocked.html       # Die App-Blocker Verwaltung
