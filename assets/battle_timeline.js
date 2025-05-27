class BattleTimelineGenerator {
    constructor() {
        this.battleData = null;
        this.colors = [
            'rgba(255, 99, 132, 0.9)',
            'rgba(54, 162, 235, 0.9)',
            'rgba(75, 192, 192, 0.9)',
            'rgba(255, 206, 86, 0.9)',
            'rgba(153, 102, 255, 0.9)',
            'rgba(255, 159, 64, 0.9)',
            'rgba(46, 204, 113, 0.9)',
            'rgba(192, 57, 43, 0.9)',
            'rgba(155, 89, 182, 0.9)'
        ];

        this.battleEvents = [];
        this.avatars = [];
        this.currentCycle = 0;
        this.currentWave = 0;
        this.turnCount = 0;
        this.totalDamage = 0;
        this.isListening = false;
        this.eventSource = null;
        this.pendingSkills = [];
        this.pendingDamages = [];
        this.enemies = [];
    }

    initBattleData(eventData) {
        this.battleData = eventData.data;
        return this.battleData;
    }

    initEventListener() {
        if (this.isListening) return;
        this.isListening = true;
        this.battleEvents = [];
        this.turnEvents = [];
        this.avatars = [];

        this.eventSource = new EventSource('http://localhost:1305/events');

        this.eventSource.onmessage = (event) => {
            if (event.data === 'ping') return;

            try {
                const eventData = JSON.parse(event.data);
                this.processEvent(eventData);
            } catch (e) {
            }
        };

        this.eventSource.onerror = (error) => {
            this.isListening = false;
            setTimeout(() => {
                if (!this.isListening) {
                    this.initEventListener();
                }
            }, 5000);
        };
    }

    processEvent(eventData) {
        const eventType = eventData.type;
        const data = eventData.data;

        switch (eventType) {
            case 'OnSetBattleLineup':
                this.avatars = data.avatars;
                this.battleEvents = [];
                this.turnEvents = [];
                this.totalDamage = 0;
                this.turnCount = 0;
                this.battleData = null;
                this.pendingSkills = [];
                this.pendingDamages = [];
                this.enemies = [];
                this.currentTurn = {
                    owner: null,
                    skills: [],
                    damages: [],
                    actionValue: 0
                };
                // Clear enemy summary display
                this.createEnemySummary();
                break;

            case 'OnBattleBegin':
                this.battleEvents = [];
                this.turnEvents = [];
                this.totalDamage = 0;
                this.turnCount = 0;
                this.pendingSkills = [];
                this.pendingDamages = [];
                this.currentTurn = {
                    owner: null,
                    skills: [],
                    damages: [],
                    actionValue: 0
                };

                this.max_waves = data.max_waves || 0;
                this.max_cycles = data.max_cycles || 0;
                this.stage_id = data.stage_id || 0;
                this.battleData = {
                    max_waves: this.max_waves,
                    max_cycles: this.max_cycles,
                    stage_id: this.stage_id,
                    turn_count: 0,
                    total_damage: 0,
                    cycle: this.currentCycle,
                    wave: this.currentWave
                };

                this.updateBattleInfo();
                break;

            case 'OnTurnBegin':
                if (this.currentTurn.owner) {
                    const completeTurn = {
                        type: 'TurnComplete',
                        owner: this.currentTurn.owner,
                        skills: [...this.currentTurn.skills],
                        damages: [...this.currentTurn.damages],
                        actionValue: this.currentTurn.actionValue,
                        turnNumber: this.turnCount,
                        monster_hps: this.currentTurn.monster_hps || [],
                        timestamp: new Date()
                    };
                    this.battleEvents.push(completeTurn);
                    this.turnEvents.push(completeTurn);
                }

                this.currentTurn = {
                    owner: data.turn_owner,
                    skills: [...this.pendingSkills],
                    damages: [...this.pendingDamages],
                    actionValue: data.action_value,
                    total_elapsed_action_value: data.action_value,
                    monster_hps: data.monster_hps || []
                };

                this.pendingSkills = [];
                this.pendingDamages = [];

                this.battleEvents.push({
                    type: 'TurnBegin',
                    data: data,
                    timestamp: new Date()
                });
                break;

            case 'OnTurnEnd':
                this.battleEvents.push({
                    type: 'TurnEnd',
                    data: data,
                    timestamp: new Date()
                });

                const completeTurn = {
                    type: 'TurnComplete',
                    owner: this.currentTurn.owner,
                    skills: [...this.currentTurn.skills],
                    damages: [...this.currentTurn.damages],
                    actionValue: this.currentTurn.actionValue,
                    total_elapsed_action_value: this.currentTurn.total_elapsed_action_value,
                    turnNumber: this.turnCount,
                    turnInfo: data.turn_info,
                    monster_hps: this.currentTurn.monster_hps || [],
                    timestamp: new Date()
                };

                this.battleEvents.push(completeTurn);
                this.turnEvents.push(completeTurn);

                this.currentTurn = {
                    owner: null,
                    skills: [],
                    damages: [],
                    actionValue: 0,
                    total_elapsed_action_value: 0
                };

                this.turnCount++;
                this.generateTimelineFromEvents();
                break;

            case 'OnUseSkill':
                const isUltimate = data.skill && data.skill.type === 'Ultimate';

                if (!this.currentTurn.owner && isUltimate) {
                    this.pendingSkills.push(data);
                } else {
                    this.currentTurn.skills.push(data);
                }

                this.battleEvents.push({
                    type: 'UseSkill',
                    data: data,
                    timestamp: new Date()
                });
                break;

            case 'OnDamage':
                const isFollowUp = data.damage_type === 'Follow-up';

                if (!this.currentTurn.owner && isFollowUp) {
                    this.pendingDamages.push(data);
                } else {
                    this.currentTurn.damages.push(data);
                }

                this.battleEvents.push({
                    type: 'Damage',
                    data: data,
                    timestamp: new Date()
                });

                if (data && typeof data.damage === 'number' && !isNaN(data.damage)) {
                    this.totalDamage += data.damage;
                } else {
                }
                break;

            case 'OnUpdateCycle':
                this.currentCycle = data.cycle;
                break;

            case 'OnUpdateWave':
                this.currentWave = data.wave;
                break;

            case 'OnBattleEnd':
                if (data.cycle) this.currentCycle = data.cycle;
                if (data.wave) this.currentWave = data.wave;
                if (data.max_waves) this.max_waves = data.max_waves;
                if (data.max_cycles) this.max_cycles = data.max_cycles;
                if (data.stage_id) this.stage_id = data.stage_id;

                if (this.currentTurn.owner) {
                    const completeTurn = {
                        type: 'TurnComplete',
                        owner: this.currentTurn.owner,
                        skills: [...this.currentTurn.skills],
                        damages: [...this.currentTurn.damages],
                        actionValue: this.currentTurn.actionValue,
                        total_elapsed_action_value: this.currentTurn.total_elapsed_action_value,
                        turnNumber: this.turnCount,
                        monster_hps: this.currentTurn.monster_hps || [],
                        timestamp: new Date()
                    };
                    this.battleEvents.push(completeTurn);
                    this.turnEvents.push(completeTurn);
                }

                this.generateTimelineFromEvents();
                break;

            case 'OnInitializeEnemy':
                if (data && data.enemy) {
                    this.enemies.push(data.enemy);
                    this.createEnemySummary();
                }
                break;
        }
    }

    generateTimelineFromEvents() {
        if (this.turnEvents.length === 0) {
            return;
        }

        const syntheticData = {
            avatars: this.avatars,
            turn_history: [],
            turn_count: this.turnCount,
            total_damage: this.totalDamage,
            cycle: this.currentCycle,
            wave: this.currentWave,
            max_waves: this.max_waves || 0,
            max_cycles: this.max_cycles || 0,
            stage_id: this.stage_id || 0
        };

        for (const event of this.turnEvents) {
            if (event.turnInfo) {
                syntheticData.turn_history.push({
                    ...event.turnInfo,
                    owner: event.owner,
                    skills: event.skills,
                    damages: event.damages,
                    total_elapsed_action_value: event.total_elapsed_action_value || event.turnInfo.action_value
                });
            } else {
                const avatarsTurnDamage = new Array(syntheticData.avatars.length).fill(0);
                let totalDamage = 0;

                if (event.damages && event.damages.length > 0) {
                    for (const damage of event.damages) {
                        const avatarIndex = syntheticData.avatars.findIndex(a =>
                            a.id === damage.data.attacker.id);

                        if (avatarIndex >= 0) {
                            avatarsTurnDamage[avatarIndex] += damage.data.damage;
                            totalDamage += damage.data.damage;
                        }
                    }
                }

                syntheticData.turn_history.push({
                    total_elapsed_action_value: event.total_elapsed_action_value || 0,
                    action_value: event.actionValue || 0,
                    cycle: this.currentCycle,
                    wave: this.currentWave,
                    avatars_turn_damage: avatarsTurnDamage,
                    total_damage: totalDamage,
                    owner: event.owner,
                    skills: event.skills,
                    damages: event.damages,
                    turnNumber: event.turnNumber,
                    monster_hps: event.monster_hps || []
                });
            }
        }

        this.battleData = syntheticData;

        this.updateBattleInfo();
        this.initializeBattleFlow();

        const timelineContainer = document.getElementById('timeline-container');
        if (timelineContainer) {
            timelineContainer.style.display = 'block';
        }
    }

    stopEventListener() {
        if (this.eventSource) {
            this.eventSource.close();
            this.isListening = false;
        }
    }

    formatDamage(damage) {
        return Math.round(damage).toLocaleString();
    }

    checkUltimateSkill(avatarId, turnIndex) {
        if (!this.battleData || !this.battleData.turn_history || turnIndex < 0 || turnIndex >= this.battleData.turn_history.length) {
            return false;
        }

        const turn = this.battleData.turn_history[turnIndex];
        if (!turn || !turn.skills || turn.skills.length === 0) {
            return false;
        }

        const avatarSkills = turn.skills.filter(skill => {
            return (skill.avatar &&
                skill.avatar.id === avatarId &&
                skill.skill &&
                skill.skill.type === 'Ultimate') ||
                (skill.avatar &&
                    skill.avatar.id === avatarId &&
                    skill.type === 'Ultimate');
        });

        return avatarSkills.length > 0;
    }

    checkFollowUpDamage(avatarId, turnIndex) {
        if (!this.battleData || !this.battleData.turn_history || turnIndex < 0 || turnIndex >= this.battleData.turn_history.length) {
            return 0;
        }

        const turn = this.battleData.turn_history[turnIndex];
        if (!turn || !turn.damages || turn.damages.length === 0) {
            return 0;
        }

        const followUpDamages = turn.damages.filter(dmg => {
            return (dmg.data &&
                dmg.data.attacker &&
                dmg.data.attacker.id === avatarId &&
                dmg.data.damage_type === 'Follow-up') ||
                (dmg.attacker &&
                    dmg.attacker.id === avatarId &&
                    dmg.damage_type === 'Follow-up');
        });

        return followUpDamages.length;
    }

    checkNormalSkill(avatarId, turnIndex) {
        if (!this.battleData || !this.battleData.turn_history || turnIndex < 0 || turnIndex >= this.battleData.turn_history.length) {
            return false;
        }

        const turn = this.battleData.turn_history[turnIndex];
        if (!turn || !turn.skills || turn.skills.length === 0) {
            return false;
        }

        const normalSkills = turn.skills.filter(skill => {
            return (skill.avatar &&
                skill.avatar.id === avatarId &&
                skill.skill &&
                skill.skill.type === 'Skill') ||
                (skill.avatar &&
                    skill.avatar.id === avatarId &&
                    skill.type === 'Skill');
        });

        return normalSkills.length > 0;
    }

    checkServantSkill(avatarId, turnIndex) {
        if (!this.battleData || !this.battleData.turn_history || turnIndex < 0 || turnIndex >= this.battleData.turn_history.length) {
            return false;
        }

        const turn = this.battleData.turn_history[turnIndex];
        if (!turn || !turn.skills || turn.skills.length === 0) {
            return false;
        }

        const servantSkills = turn.skills.filter(skill => {
            return (skill.avatar &&
                skill.avatar.id === avatarId &&
                skill.skill &&
                skill.skill.type === 'Servant') ||
                (skill.avatar &&
                    skill.avatar.id === avatarId &&
                    skill.type === 'Servant');
        });

        return servantSkills.length > 0;
    }

    // Check Pull
    checkActionValuePull(avatarId, turnIndex) {
        if (avatarId != 1101 && avatarId != 1313) {
            return false;
        }

        if (!this.battleData || !this.battleData.turn_history || turnIndex < 0 || turnIndex >= this.battleData.turn_history.length - 1) {
            return false;
        }

        const currentTurn = this.battleData.turn_history[turnIndex];
        const nextTurn = this.battleData.turn_history[turnIndex + 1];

        if (!currentTurn || !nextTurn || !currentTurn.owner || !nextTurn.owner) {
            return false;
        }

        const usedNormalSkill = this.checkNormalSkill(avatarId, turnIndex);

        if (usedNormalSkill && currentTurn.owner.id === avatarId) {
            return true;
        }

        return false;
    }

    // Check Servant Pull
    checkServantPull(avatarId, turnIndex) {
        if (avatarId != 8007 && avatarId != 8008) {
            return false;
        }

        if (!this.battleData || !this.battleData.turn_history || turnIndex < 0 || turnIndex >= this.battleData.turn_history.length - 1) {
            return false;
        }

        const currentTurn = this.battleData.turn_history[turnIndex];
        const nextTurn = this.battleData.turn_history[turnIndex + 1];

        if (!currentTurn || !nextTurn || !currentTurn.owner || !nextTurn.owner) {
            return false;
        }

        const usedServantSkill = this.checkServantSkill(avatarId, turnIndex);

        if (usedServantSkill && currentTurn.owner.id === avatarId) {
            return true;
        }

        return false;
    }

    getAvatarNameById(id) {
        if (id === 'start') {
            return 'Start';
        }

        let avatar = this.battleData.avatars.find(avatar => avatar.id == id);
        if (!avatar && this.enemies && this.enemies.length > 0) {
            avatar = this.enemies.find(e => e.id == id);
        }
        if (avatar && avatar.name) {
            return avatar.name;
        }

        return `ID: ${id}`;
    }

    createNode(id, type, x, y, label, damage, actionValue, positionIndex, ultimate = false, followUpCount = 0, nodeType = '') {
        const node = document.createElement('div');
        let styleClass = type === 'start' ? 'start' : `pos-${positionIndex + 1}`;

        if (nodeType === 'owner') {
            styleClass += ' turn-owner';
        } else if (nodeType === 'skill') {
            styleClass += ' skill-user';
        }

        node.className = `node node-${styleClass}`;
        node.id = id;
        node.style.left = `${x}px`;
        node.style.top = `${y}px`;

        let content = '';

        if (type === 'start') {
            content = `<div>Start</div>`;
        } else {
            const avatarName = this.getAvatarNameById(type);
            content = `
                <div class="node-label">${avatarName}</div>
            `;

            if (nodeType === 'owner') {
                content += `<div class="node-owner-indicator">Turn Owner</div>`;
            }

            if (damage > 0) {
                content += `<div class="node-damage">${this.formatDamage(damage)}</div>`;
            } else if (nodeType === 'owner') {
                content += `<div class="node-no-damage">No Damage</div>`;
            }

            if (actionValue) {
                content += `<div class="node-av">AV: ${actionValue.toFixed(1)}</div>`;
            }
        }

        node.innerHTML = content;

        if (ultimate) {
            const ultimateIndicator = document.createElement('div');
            ultimateIndicator.className = 'node-ultimate-indicator';
            ultimateIndicator.textContent = 'Ultimate';
            node.appendChild(ultimateIndicator);
        }

        if (followUpCount > 0) {
            const followUpIndicator = document.createElement('div');
            followUpIndicator.className = 'node-followup-indicator';
            followUpIndicator.textContent = 'Follow-up';
            node.appendChild(followUpIndicator);
        }

        node.addEventListener('mouseenter', (e) => {
            const tooltip = document.getElementById('tooltip');
            const avatarName = type === 'start' ? 'Start' : this.getAvatarNameById(type);

            let tooltipContent = `
                <div>${avatarName} ${type !== 'start' ? `(ID: ${type})` : ''}</div>
                ${nodeType === 'owner' ? '<div class="tooltip-owner">Turn Owner</div>' : ''}
                ${damage > 0 ? `<div>Damage: ${this.formatDamage(damage)}</div>` : '<div>No Damage</div>'}
                ${actionValue ? `<div>AV: ${actionValue.toFixed(2)}</div>` : ''}
                ${label ? `<div>Turn: ${label}</div>` : ''}
                ${this.battleData.cycle !== undefined ? `<div>Cycle: ${this.battleData.cycle}</div>` : ''}
                ${this.battleData.wave !== undefined ? `<div>Wave: ${this.battleData.wave}</div>` : ''}
            `;

            // Add skill information
            const turn = this.battleData.turn_history[label - 1];
            if (turn && turn.skills && turn.skills.length > 0) {
                tooltipContent += `<div class="tooltip-skills">Skills Used:</div>`;
                turn.skills.forEach(skill => {
                    if (skill.avatar && skill.avatar.id == type) {
                        tooltipContent += `<div>- ${skill.skill ? skill.skill.type : 'Skill'}</div>`;
                    }
                });
            }

            tooltip.innerHTML = tooltipContent;
            tooltip.style.left = `${e.pageX + 10}px`;
            tooltip.style.top = `${e.pageY + 10}px`;
            tooltip.style.opacity = '1';
        });

        node.addEventListener('mouseleave', () => {
            const tooltip = document.getElementById('tooltip');
            tooltip.style.opacity = '0';
        });

        return node;
    }

    createConnector(startX, startY, endX, endY, _type, positionIndex) {
        const container = document.getElementById('battle-flow');

        const dx = endX - startX;
        const dy = endY - startY;
        const length = Math.sqrt(dx * dx + dy * dy);
        const angle = Math.atan2(dy, dx) * 180 / Math.PI;

        const shortenedLength = length - 5;
        const connector = document.createElement('div');

        connector.className = `connector connector-pos-${positionIndex + 1}`;
        connector.style.width = `${shortenedLength}px`;
        connector.style.left = `${startX}px`;
        connector.style.top = `${startY}px`;
        connector.style.transform = `rotate(${angle}deg)`;

        // create arrow
        const arrow = document.createElement('div');
        arrow.className = `connector-arrow connector-arrow-pos-${positionIndex + 1}`;
        arrow.style.left = `${shortenedLength - 6}px`;
        arrow.style.top = '-4px';

        connector.appendChild(arrow);
        container.appendChild(connector);
    }

    // Pull Line
    createAVPullConnector(startX, startY, endX, endY, positionIndex = 0) {
        const container = document.getElementById('battle-flow');

        const pullerPositionIndex = positionIndex;
        const startCenterX = startX;
        const endCenterX = endX;
        const nodeHeight = 80;
        const verticalDropDistance = 30;

        // line
        const verticalDropConnector = document.createElement('div');
        verticalDropConnector.className = `connector connector-pos-${pullerPositionIndex + 1}`;
        verticalDropConnector.style.opacity = '0.5';

        verticalDropConnector.style.width = `${verticalDropDistance}px`;
        verticalDropConnector.style.left = `${startCenterX}px`;
        verticalDropConnector.style.top = `${startY + nodeHeight}px`;
        verticalDropConnector.style.transform = 'rotate(90deg)';
        verticalDropConnector.style.transformOrigin = '0 0';

        const horizontalConnector = document.createElement('div');
        horizontalConnector.className = `connector connector-pos-${pullerPositionIndex + 1}`;
        horizontalConnector.style.opacity = '0.5';

        const horizontalLength = Math.abs(endCenterX - startCenterX);
        const horizontalDirection = endCenterX > startCenterX ? 1 : -1;

        horizontalConnector.style.width = `${horizontalLength}px`;
        horizontalConnector.style.left = horizontalDirection > 0 ? `${startCenterX}px` : `${endCenterX}px`;
        horizontalConnector.style.top = `${startY + nodeHeight + verticalDropDistance}px`;
        horizontalConnector.style.transform = 'rotate(0deg)';

        const verticalRiseConnector = document.createElement('div');
        verticalRiseConnector.className = `connector connector-pos-${pullerPositionIndex + 1}`;
        verticalRiseConnector.style.opacity = '0.5';
        const verticalRiseDistance = Math.abs((endY + nodeHeight) - (startY + nodeHeight + verticalDropDistance));
        verticalRiseConnector.style.width = `${verticalRiseDistance}px`;
        verticalRiseConnector.style.left = `${endCenterX}px`;
        const verticalRiseStartY = Math.min(startY + nodeHeight + verticalDropDistance, endY + nodeHeight);
        verticalRiseConnector.style.top = `${verticalRiseStartY}px`;
        const verticalDirection = (endY + nodeHeight) > (startY + nodeHeight + verticalDropDistance) ? 1 : -1;
        verticalRiseConnector.style.transform = verticalDirection > 0 ? 'rotate(90deg)' : 'rotate(270deg)';
        verticalRiseConnector.style.transformOrigin = '0 0';

        const pullText = document.createElement('div');
        pullText.style.position = 'absolute';
        pullText.style.zIndex = '50';
        pullText.style.fontSize = '12px';
        pullText.style.fontWeight = 'bold';
        pullText.style.backgroundColor = 'white';
        pullText.style.padding = '2px 5px';
        pullText.style.borderRadius = '3px';
        pullText.style.boxShadow = '0 1px 3px rgba(0,0,0,0.2)';

        let pullColor = '#666';
        if (pullerPositionIndex === 0) pullColor = 'rgba(255, 99, 132, 0.9)';
        else if (pullerPositionIndex === 1) pullColor = 'rgba(54, 162, 235, 0.9)';
        else if (pullerPositionIndex === 2) pullColor = 'rgba(75, 192, 192, 0.9)';
        else if (pullerPositionIndex === 3) pullColor = 'rgba(255, 206, 86, 0.9)';

        pullText.style.border = '1px solid ' + pullColor;
        pullText.style.color = pullColor;
        pullText.innerHTML = 'Pull';
        pullText.style.pointerEvents = 'none';

        const pullLabelX = horizontalDirection > 0 ?
            startCenterX + horizontalLength / 2 - 15 :
            endCenterX + horizontalLength / 2 - 15;

        pullText.style.left = `${pullLabelX}px`;
        pullText.style.top = `${startY + nodeHeight + verticalDropDistance - 10}px`;

        container.appendChild(verticalDropConnector);
        container.appendChild(horizontalConnector);
        container.appendChild(verticalRiseConnector);
        container.appendChild(pullText);
    }

    initializeBattleFlow() {
        if (!this.battleData) {
            const container = document.getElementById('battle-flow');
            if (container) {
                container.innerHTML = '<div style="text-align: center; padding: 20px; width: 80%; max-width: 500px; background-color: rgba(0,0,0,0.03); border-radius: 10px; font-size: 16px; color: #555;">Waiting for battle data...<br>Start a battle to see the timeline.</div>';
            }
            return;
        }
        const TURN_HEIGHT = 210;
        const VISIBLE_TURN_COUNT = 8;
        let viewport = document.getElementById('battle-flow-viewport');
        if (!viewport) {
            const old = document.getElementById('battle-flow');
            viewport = document.createElement('div');
            viewport.id = 'battle-flow-viewport';
            viewport.style.position = 'relative';
            viewport.style.width = '100%';
            if (old) {
                old.parentNode.insertBefore(viewport, old);
                viewport.appendChild(old);
            }
        }
        viewport.style.overflowY = 'auto';
        viewport.style.height = (TURN_HEIGHT * VISIBLE_TURN_COUNT) + 'px';
        this.renderVirtualTurns();
        if (!viewport._virtualScrollBinded) {
            viewport.addEventListener('scroll', () => {
                this.renderVirtualTurns();
            });
            viewport._virtualScrollBinded = true;
        }
    }

    renderVirtualTurns() {
        const TURN_HEIGHT = 210;
        const VISIBLE_TURN_COUNT = 8;
        const container = document.getElementById('battle-flow');
        const viewport = document.getElementById('battle-flow-viewport');
        if (!container || !viewport || !this.battleData) return;
        const avatars = this.battleData.avatars;
        const turnHistory = this.battleData.turn_history;
        const turnCount = turnHistory.length;
        const scrollTop = viewport.scrollTop;
        const start = Math.max(0, Math.floor(scrollTop / TURN_HEIGHT));
        const end = Math.min(turnCount, start + VISIBLE_TURN_COUNT);
        const VERTICAL_SPACING = TURN_HEIGHT;
        const CONTAINER_WIDTH = Math.min(container.clientWidth || 1200, 1600);
        const LANE_COUNT = avatars.length;
        const lanes = {};
        const leftPadding = 300;
        const rightPadding = 250;
        const usableWidth = CONTAINER_WIDTH - leftPadding - rightPadding;
        const laneSpacing = LANE_COUNT > 1 ? usableWidth / (LANE_COUNT - 1) : usableWidth;
        avatars.forEach((avatar, index) => {
            if (LANE_COUNT === 1) {
                lanes[avatar.id] = (CONTAINER_WIDTH / 2);
            } else {
                lanes[avatar.id] = leftPadding + (index * laneSpacing);
            }
        });
        const startNodes = {};
        avatars.forEach((avatar, avatarIndex) => {
            const x = lanes[avatar.id];
            const y = 60;
            const nodeId = `start-${avatar.id}`;
            const node = this.createNode(nodeId, 'start', x, y, 'Start', 0, null, avatarIndex);
            startNodes[avatar.id] = { x: x, y: y + 35, node: node, positionIndex: avatarIndex };
        });
        container.innerHTML = '';
        let lastNodePositions = {};
        avatars.forEach((avatar, avatarIndex) => {
            container.appendChild(startNodes[avatar.id].node);
            lastNodePositions[avatar.id] = { x: startNodes[avatar.id].x, y: startNodes[avatar.id].y };
        });
        let currentY = 180 + start * VERTICAL_SPACING;
        for (let turnIndex = start; turnIndex < end; turnIndex++) {
            const turn = turnHistory[turnIndex];
            const isMonsterTurn = turn.owner && turn.owner.id >= 20000;
            // turn label
            const turnLabel = document.createElement('div');
            turnLabel.className = 'turn-label';
            let turnLabelHtml = `Turn ${turnIndex + 1}<br>AV: ${(turn.relative_action_value || turn.action_value || 0).toFixed(1)}`;

            // Append monster HPs if present
            if (turn.monster_hps && turn.monster_hps.length > 0) {
                turn.monster_hps.forEach(m => {
                    const enemyEntry = this.enemies.find(e => e.id === m.monster_id);
                    const maxHp = enemyEntry && enemyEntry.base_stats ? enemyEntry.base_stats.hp : 0;
                    const hpText = maxHp ? `${this.formatDamage(m.hp)} / ${this.formatDamage(maxHp)}` : this.formatDamage(m.hp);
                    turnLabelHtml += `<br>ID: ${m.monster_id}<br>${hpText}`;
                });
            }

            turnLabel.innerHTML = turnLabelHtml;
            turnLabel.style.top = `${currentY - 15}px`;
            turnLabel.style.left = '20px';
            turnLabel.style.width = '140px';
            container.appendChild(turnLabel);
            avatars.forEach((avatar, avatarIndex) => {
                const damage = turn.avatars_turn_damage[avatarIndex];
                const isOwner = turn.owner && turn.owner.id === avatar.id;
                if (isMonsterTurn) {
                    const x = lanes[avatar.id];
                    const y = currentY;
                    const nodeId = `monster-turn-${turnIndex + 1}-char-${avatar.id}`;
                    const node = this.createNode(
                        nodeId,
                        turn.owner.id,
                        x,
                        y,
                        turnIndex + 1,
                        turn.monster_hps && turn.monster_hps.length > 0 ? turn.monster_hps.find(m => m.monster_id === turn.owner.id).hp : 0,
                        turn.relative_action_value || turn.action_value || 0,
                        avatarIndex,
                        false,
                        0,
                        'Monster'
                    );
                    container.appendChild(node);
                    if (lastNodePositions[avatar.id]) {
                        const lastPos = lastNodePositions[avatar.id];
                        this.createConnector(lastPos.x, lastPos.y, x, y, avatar.id, avatarIndex);
                    } else {
                        const startPos = startNodes[avatar.id];
                        this.createConnector(startPos.x, startPos.y, x, y, avatar.id, avatarIndex);
                    }
                    lastNodePositions[avatar.id] = { x: x, y: y + 35 };
                    return;
                }
                if (isOwner || damage > 0) {
                    const x = lanes[avatar.id];
                    const y = currentY;
                    const nodeId = `turn-${turnIndex + 1}-char-${avatar.id}`;
                    const hasUltimate = this.checkUltimateSkill(avatar.id, turnIndex);
                    const followUpCount = this.checkFollowUpDamage(avatar.id, turnIndex);
                    const node = this.createNode(
                        nodeId,
                        avatar.id,
                        x,
                        y,
                        turnIndex + 1,
                        damage,
                        turn.relative_action_value || turn.action_value || 0,
                        avatarIndex,
                        hasUltimate,
                        followUpCount,
                        isOwner ? 'owner' : 'damage'
                    );
                    container.appendChild(node);
                    if (lastNodePositions[avatar.id]) {
                        const lastPos = lastNodePositions[avatar.id];
                        this.createConnector(lastPos.x, lastPos.y, x, y, avatar.id, avatarIndex);
                    } else {
                        const startPos = startNodes[avatar.id];
                        this.createConnector(startPos.x, startPos.y, x, y, avatar.id, avatarIndex);
                    }
                    lastNodePositions[avatar.id] = { x: x, y: y + 35 };
                    if ((avatar.id === 1101 || avatar.id === 1313) && turnIndex < turnHistory.length - 1) {
                        const isPull = this.checkActionValuePull(avatar.id, turnIndex);
                        if (isPull) {
                            const nextTurn = turnHistory[turnIndex + 1];
                            if (nextTurn && nextTurn.owner) {
                                const nextAvatarId = nextTurn.owner.id;
                                const nextX = lanes[nextAvatarId];
                                const nextY = currentY + VERTICAL_SPACING;
                                const pullerAvatarIndex = avatarIndex;
                                this.createAVPullConnector(x, y, nextX, nextY, pullerAvatarIndex);
                            }
                        }
                    }
                    if ((avatar.id === 8007 || avatar.id === 8008) && turnIndex < turnHistory.length - 1) {
                        const isServantPull = this.checkServantPull(avatar.id, turnIndex);
                        if (isServantPull) {
                            const nextTurn = turnHistory[turnIndex + 1];
                            if (nextTurn && nextTurn.owner) {
                                const nextAvatarId = nextTurn.owner.id;
                                const nextX = lanes[nextAvatarId];
                                const nextY = currentY + VERTICAL_SPACING;
                                const pullerAvatarIndex = avatarIndex;
                                this.createAVPullConnector(x, y, nextX, nextY, pullerAvatarIndex);
                            }
                        }
                    }
                }
            });
            currentY += VERTICAL_SPACING;
        }
        container.style.paddingTop = `${start * VERTICAL_SPACING}px`;
        container.style.paddingBottom = `${(turnCount - end) * VERTICAL_SPACING}px`;
        container.style.minHeight = `${turnCount * VERTICAL_SPACING + 100}px`;
        container.style.height = 'auto';
        container.style.maxHeight = 'none';
        this.createSummary();
    }

    createSummary() {
        const summaryContent = document.getElementById('summary-content');
        summaryContent.innerHTML = '';

        const characterSummary = this.battleData.avatars.map((avatar, index) => {
            let totalDamage = 0;
            let damageActionTurns = 0;
            let totalTurns = 0;
            let skillsUsed = [];

            this.battleData.turn_history.forEach(turn => {
                if (turn.avatars_turn_damage && turn.avatars_turn_damage[index] > 0) {
                    damageActionTurns++;
                    totalDamage += turn.avatars_turn_damage[index];
                }

                if (turn.owner && turn.owner.id === avatar.id) {
                    totalTurns++;
                }

                if (turn.skills && turn.skills.length > 0) {
                    turn.skills.forEach(skill => {
                        if (skill.avatar && skill.avatar.id === avatar.id) {
                            skillsUsed.push(skill.skill ? skill.skill.type : 'Skill');
                        }
                    });
                }
            });

            return {
                avatar: avatar,
                totalDamage: totalDamage,
                damageActionTurns: damageActionTurns,
                totalTurns: totalTurns,
                skillsUsed: skillsUsed,
                originalIndex: index
            };
        });

        let totalBattleDamage = 0;
        characterSummary.forEach(data => {
            totalBattleDamage += data.totalDamage;
        });
        this.battleData.total_damage = totalBattleDamage;

        // Sort by total damage (highest first)
        characterSummary.sort((a, b) => b.totalDamage - a.totalDamage);

        characterSummary.forEach((data) => {
            const summaryItem = document.createElement('div');
            summaryItem.className = 'summary-item';
            const positionIndex = data.originalIndex;
            const color = this.colors[positionIndex % this.colors.length];

            const characterDiv = document.createElement('div');
            characterDiv.className = 'summary-character';

            const colorDot = document.createElement('span');
            colorDot.className = 'color-dot';
            colorDot.style.backgroundColor = color;

            const nameSpan = document.createElement('span');
            nameSpan.className = 'attacker-name';
            nameSpan.textContent = this.getAvatarNameById(data.avatar.id);

            characterDiv.appendChild(colorDot);
            characterDiv.appendChild(nameSpan);

            const damageStats = document.createElement('div');
            damageStats.className = 'damage-stats';

            if (this.battleData.total_damage > 0 && data.totalDamage > 0) {
                let percentage = (data.totalDamage / this.battleData.total_damage * 100).toFixed(1);
                if (parseFloat(percentage) > 100) {
                    percentage = "100.0";
                }
                const percentageDiv = document.createElement('div');
                percentageDiv.className = 'damage-percentage';
                percentageDiv.textContent = `${percentage}%`;
                summaryItem.appendChild(percentageDiv);
            }
            const damageValueDiv = document.createElement('div');
            damageValueDiv.className = 'damage-value';
            damageValueDiv.textContent = data.totalDamage > 0 ?
                `${this.formatDamage(data.totalDamage)}` : 'No Damage';

            const turnCountDiv = document.createElement('div');
            turnCountDiv.className = 'turn-count';

            if (data.totalTurns > 0) {
                turnCountDiv.textContent = `Turns: ${data.totalTurns}`;
                if (data.damageActionTurns > 0) {
                    turnCountDiv.textContent += ` (${data.damageActionTurns} with damage)`;
                }
            } else {
                turnCountDiv.textContent = 'No Turns';
            }

            // Skills used
            if (data.skillsUsed.length > 0) {
                const skillsDiv = document.createElement('div');
                skillsDiv.className = 'skills-used';

                const skillCounts = {};
                data.skillsUsed.forEach(skill => {
                    skillCounts[skill] = (skillCounts[skill] || 0) + 1;
                });

                const skillText = Object.entries(skillCounts)
                    .map(([skill, count]) => `${skill}: ${count}`)
                    .join(', ');

                skillsDiv.textContent = `${skillText}`;
                damageStats.appendChild(skillsDiv);
            }

            damageStats.appendChild(damageValueDiv);
            damageStats.appendChild(turnCountDiv);

            summaryItem.appendChild(characterDiv);
            summaryItem.appendChild(damageStats);

            summaryContent.appendChild(summaryItem);
        });

        this.createEnemySummary();
    }

    createEnemySummary() {
        const enemySummaryBox = document.getElementById('enemy-summary-box');
        const enemySummaryContent = document.getElementById('enemy-summary-content');
        if (!enemySummaryBox || !enemySummaryContent) return;

        enemySummaryContent.innerHTML = '';

        if (!this.enemies || this.enemies.length === 0) {
            enemySummaryBox.style.display = 'none';
            return;
        }

        enemySummaryBox.style.display = 'block';

        this.enemies.forEach((enemy) => {
            if (!enemy) return;

            const item = document.createElement('div');
            item.className = 'summary-item';

            const nameDiv = document.createElement('div');
            nameDiv.className = 'enemy-name';
            nameDiv.textContent = enemy.name || 'Unknown';

            const hp = enemy.base_stats && typeof enemy.base_stats.hp === 'number' ? this.formatDamage(enemy.base_stats.hp) : 'N/A';
            const hpDiv = document.createElement('div');
            hpDiv.className = 'enemy-hp';
            hpDiv.textContent = `HP: ${hp}`;

            const level = enemy.base_stats && enemy.base_stats.level !== undefined ? enemy.base_stats.level : 'N/A';
            const levelDiv = document.createElement('div');
            levelDiv.className = 'turn-count';
            levelDiv.textContent = `Level: ${level}`;

            const idDiv = document.createElement('div');
            idDiv.className = 'turn-count';
            idDiv.textContent = `ID: ${enemy.id}`;

            const uidDiv = document.createElement('div');
            uidDiv.className = 'turn-count';
            uidDiv.textContent = `UID: ${enemy.uid}`;

            item.appendChild(nameDiv);
            item.appendChild(hpDiv);
            item.appendChild(levelDiv);
            item.appendChild(idDiv);
            item.appendChild(uidDiv);

            enemySummaryContent.appendChild(item);
        });
    }

    // Battle Turn info
    updateBattleInfo() {
        if (this.battleData.max_cycles !== undefined) {
            document.getElementById('max-cycles-info').textContent = `Max Cycles: ${this.battleData.max_cycles}`;
        }

        if (this.battleData.max_waves !== undefined) {
            document.getElementById('max-waves-info').textContent = `Max Waves: ${this.battleData.max_waves}`;
        }

        if (this.battleData.cycle !== undefined) {
            document.getElementById('cycle-info').textContent = `Cycle: ${this.battleData.cycle}`;
        }

        if (this.battleData.wave !== undefined) {
            document.getElementById('wave-info').textContent = `Wave: ${this.battleData.wave}`;
        }

        if (this.battleData.turn_count !== undefined) {
            document.getElementById('turn-count').textContent = `AllTurn: ${this.battleData.turn_count}`;
        }

        if (this.battleData.stage_id !== undefined) {
            document.getElementById('stage-id-info').textContent = `Stage ID: ${this.battleData.stage_id}`;
        }
    }

    // Show Timeline
    showBattleTimeline(eventData) {
        if (eventData && eventData.data) {
            if (eventData.data.cycle) this.currentCycle = eventData.data.cycle;
            if (eventData.data.wave) this.currentWave = eventData.data.wave;

            if (this.avatars.length === 0 && eventData.data.avatars) {
                this.avatars = eventData.data.avatars;
            }
        }

        if (this.turnEvents && this.turnEvents.length > 0) {
            this.generateTimelineFromEvents();
        } else {
            const timelineContainer = document.getElementById('timeline-container');
            if (timelineContainer) {
                timelineContainer.style.display = 'block';
            }
        }
    }

    startListening() {
        if (!this.isListening) {
            this.battleEvents = [];
            this.turnEvents = [];
            this.totalDamage = 0;
            this.turnCount = 0;
            this.pendingSkills = [];
            this.pendingDamages = [];
            this.enemies = [];
            this.currentTurn = {
                owner: null,
                skills: [],
                damages: [],
                actionValue: 0
            };

            this.initEventListener();
        }
    }
}
window.BattleTimelineGenerator = new BattleTimelineGenerator();
