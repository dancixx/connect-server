-- Interests
DELETE FROM i18n_interest WHERE id IN (
    'i18n_interest:1', 'i18n_interest:2', 'i18n_interest:3', 'i18n_interest:4', 
    'i18n_interest:5', 'i18n_interest:6', 'i18n_interest:7', 'i18n_interest:8',
    'i18n_interest:9', 'i18n_interest:10', 'i18n_interest:11', 'i18n_interest:12',
    'i18n_interest:13', 'i18n_interest:14', 'i18n_interest:15', 'i18n_interest:16',
    'i18n_interest:17', 'i18n_interest:18', 'i18n_interest:19', 'i18n_interest:20',
    'i18n_interest:21', 'i18n_interest:22', 'i18n_interest:23', 'i18n_interest:24',
    'i18n_interest:25', 'i18n_interest:26', 'i18n_interest:27', 'i18n_interest:28',
    'i18n_interest:29', 'i18n_interest:30', 'i18n_interest:31', 'i18n_interest:32',
    'i18n_interest:33', 'i18n_interest:34', 'i18n_interest:35', 'i18n_interest:36',
    'i18n_interest:37', 'i18n_interest:38', 'i18n_interest:39', 'i18n_interest:40'
);

-- Goals
DELETE FROM i18n_goal WHERE id IN (
    'i18n_goal:1', 'i18n_goal:2', 'i18n_goal:3', 'i18n_goal:4', 
    'i18n_goal:5', 'i18n_goal:6', 'i18n_goal:7', 'i18n_goal:8'
);

-- Alcohol Consumption Habits
DELETE FROM i18n_alcohol WHERE id IN ('i18n_alcohol:1', 'i18n_alcohol:2', 'i18n_alcohol:3', 'i18n_alcohol:4');

-- Education
DELETE FROM i18n_education WHERE id IN ('i18n_education:1', 'i18n_education:2', 'i18n_education:3', 'i18n_education:4', 'i18n_education:5');

-- Spoken Languages
DELETE FROM i18n_language WHERE id IN ('i18n_language:1', 'i18n_language:2', 'i18n_language:3', 'i18n_language:4', 'i18n_language:5', 'i18n_language:6', 'i18n_language:7', 'i18n_language:8');

-- Political Views
DELETE FROM i18n_political WHERE id IN ('i18n_political:1', 'i18n_political:2', 'i18n_political:3', 'i18n_political:4', 'i18n_political:5');

-- Religion
DELETE FROM i18n_religion WHERE id IN ('i18n_religion:1', 'i18n_religion:2', 'i18n_religion:3', 'i18n_religion:4', 'i18n_religion:5', 'i18n_religion:6', 'i18n_religion:7');

-- Zodiac Signs
DELETE FROM i18n_zodiac WHERE id IN ('i18n_zodiac:1', 'i18n_zodiac:2', 'i18n_zodiac:3', 'i18n_zodiac:4', 'i18n_zodiac:5', 'i18n_zodiac:6', 'i18n_zodiac:7', 'i18n_zodiac:8', 'i18n_zodiac:9', 'i18n_zodiac:10', 'i18n_zodiac:11', 'i18n_zodiac:12');
