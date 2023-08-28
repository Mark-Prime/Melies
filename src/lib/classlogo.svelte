<script>
    // @ts-nocheck
    export let player_class;
    export let tooltip = false;
    export let click = () => {};
    export let args = [];

    function getImgUrl(player_class) {
        switch (player_class) {
            case "scout":
                return "https://wiki.teamfortress.com/w/images/a/ad/Leaderboard_class_scout.png";
            case "soldier":
                return "https://wiki.teamfortress.com/w/images/9/96/Leaderboard_class_soldier.png"
            case "pyro":
                return "https://wiki.teamfortress.com/w/images/8/80/Leaderboard_class_pyro.png";
            case "demoman":
                return "https://wiki.teamfortress.com/w/images/4/47/Leaderboard_class_demoman.png"
            case "heavy":
                return "https://wiki.teamfortress.com/w/images/5/5a/Leaderboard_class_heavy.png";
            case "engineer":
                return "https://wiki.teamfortress.com/w/images/1/12/Leaderboard_class_engineer.png"
            case "medic":
                return "https://wiki.teamfortress.com/w/images/e/e5/Leaderboard_class_medic.png";
            case "sniper":
                return "https://wiki.teamfortress.com/w/images/f/fe/Leaderboard_class_sniper.png"
            case "spy":
                return "https://wiki.teamfortress.com/w/images/3/33/Leaderboard_class_spy.png";
            default:
                return false;
        }
    }
</script>

{#if getImgUrl(player_class)}
    <div 
        class={tooltip ? 'tooltip' : ''} 
        data-tooltip={tooltip} 
        style="--kills: 0"
        on:click={click(...args)}
        on:keydown={click(...args)}
    >
        <img src={getImgUrl(player_class)} alt={`${player_class} Icon`}/>
    </div>
{/if}

<style>
    img {
        height: 1.5rem;
        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>