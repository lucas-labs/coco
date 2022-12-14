import { Config, ConventionalCommitType } from '../types/coco.types';
import { existsSync, readFileSync } from 'fs';
import { homedir } from 'os';
import { parse } from 'yaml';

/** Loads user config file if it exists */
export function loadConfigFile(path: string) {
    const cfgFileNames = ['coco.yaml', 'coco.yml', '.cocorc'];
    const configPath = cfgFileNames
        .map((name) => `${path}/${name}`)
        .find((name) => existsSync(name));

    if (!configPath) return {};

    return parse(readFileSync(configPath, 'utf8')) || {};
}

/** default commit types if no types config is provided */
const defaultTypes: ConventionalCommitType[] = [
    {
        desc: 'Introduces a new feature',
        name: 'feat',
        emoji: 'โจ',
    },
    {
        desc: 'Fixes a bug',
        name: 'fix',
        emoji: '๐',
    },
    {
        desc: "Other changes that don't modify src or test files",
        name: 'chore',
        emoji: '๐งน',
    },
    {
        desc: 'Documentation only changes',
        name: 'docs',
        emoji: '๐',
    },
    {
        desc: 'Code cosmetic changes (formatting, indentation, etc.)',
        name: 'style',
        emoji: '๐',
    },
    {
        desc: 'A change that refactors code without adding or removing features',
        name: 'refactor',
        emoji: '๐จ',
    },
    {
        desc: 'A code change that improves performance',
        name: 'perf',
        emoji: '๐',
    },
    {
        desc: 'A change that only adds or updates tests',
        name: 'test',
        emoji: '๐งช',
    },
    {
        desc: 'Changes to our CI configuration files and scripts (example scopes: Travis, Circle, BrowserStack, SauceLabs)',
        name: 'ci',
        emoji: '๐',
    },
    {
        desc: 'Reverts a previous commit',
        name: 'revert',
        emoji: '๐',
    },
    {
        desc: 'Releases a new version',
        name: 'release',
        emoji: '๐',
    },
    {
        desc: 'Work in progress',
        name: 'wip',
        emoji: '๐ง',
    },
    {
        desc: 'A change that updates or adds translations (internationalization)',
        name: 'i18n',
        emoji: '๐',
    },
];

/**
 * ___Default `coco` configuration___
 *
 * We'll merge this config with the user
 * provided config
 */
const defaultConfig: Config = {
    types: defaultTypes,
    scopes: [],
    useEmoji: false,
    askScope: true,
    askBody: true,
    askFooter: true,
    askBreakingChange: true,
};

export function getConfig(repoPath: string): Config {
    const homeDir = homedir();
    let config = mergeConfig(homeDir, defaultConfig);
    config = mergeConfig(repoPath, config);

    return config;            
}

export function mergeConfig(cwd: string, cfg: Config): Config {
    const userCfg = loadConfigFile(cwd);

    const config = Object.keys(cfg)
        .map((k): keyof Config => k as keyof Config)
        .reduce((acc, key) => {
            const userOverride = userCfg[key];
            if (userOverride !== undefined) {
                acc[key] = userCfg[key];
            }
            return acc;
        }, cfg);

    return config;
}

/** Get type by its name by searching in a list of types */
export const getCommitType = (types: ConventionalCommitType[], name: string) => {
    return types.find((type) => type.name === name);
};
