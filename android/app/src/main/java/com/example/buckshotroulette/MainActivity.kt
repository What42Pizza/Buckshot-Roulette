package com.example.buckshotroulette

// Started 24/01/22

import android.annotation.SuppressLint
import android.graphics.drawable.Drawable
import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.text.Editable
import android.view.View
import android.view.ViewGroup.LayoutParams
import android.view.ViewGroup.MarginLayoutParams
import android.widget.Button
import android.widget.Space
import android.widget.TextView
import androidx.constraintlayout.widget.ConstraintLayout
import androidx.constraintlayout.widget.ConstraintSet
import com.example.buckshotroulette.databinding.ActivityMainBinding
import kotlin.math.round
import kotlin.random.Random

class MainActivity : AppCompatActivity() {

	private lateinit var binding: ActivityMainBinding

	private var data = GameData()



	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)
		binding = ActivityMainBinding.inflate(layoutInflater)
		setContentView(binding.root)

		binding.playerCountMenu.visibility = View.VISIBLE
		binding.playersListMenu.visibility = View.GONE
		binding.mainTurnMenu.visibility = View.GONE
		binding.nextTurnMenu.visibility = View.GONE
		binding.giveItemsMenu.visibility = View.GONE
		binding.reloadBuckshotMenu.visibility = View.GONE
		binding.startRoundMenu.visibility = View.GONE
		binding.startStageMenu.visibility = View.GONE
		binding.playerDataManu.visibility = View.GONE



		binding.playerCountGoButton.setOnClickListener {

			val newPlayerCount = binding.playerCountInput.text.toString().toIntOrNull()
			if (newPlayerCount == null) {
				binding.playerCountErrorLabel.text = getString(R.string.error_could_not_parse_input)
				return@setOnClickListener
			}
			if (newPlayerCount < 2) {
				binding.playerCountErrorLabel.text = getString(R.string.error_player_count_must_be_at_least_2)
				return@setOnClickListener
			}

			data.playerCount = newPlayerCount
			data.players = Array(data.playerCount) {PlayerData()}
			data.currPlayerIndex = 0

			binding.playerCountMenu.visibility = View.GONE
			binding.playerDataManu.visibility = View.VISIBLE
			reloadPlayerDataMenu()

		}



		binding.playerDataNextButton.setOnClickListener {

			val newPlayerName = binding.playerDataNameInput.text.toString()
			val newPlayerPassword = binding.playerDataPasswordInput.text.toString()
			if (newPlayerName == "") {
				binding.playerDataErrorLabel.text = getString(R.string.error_player_name_cannot_be_empty)
				return@setOnClickListener
			}
			for (player in data.players) {
				if (player.name == newPlayerName) {
					binding.playerDataErrorLabel.text = getString(R.string.error_player_name_is_already_in_use)
					return@setOnClickListener
				}
			}

			val currPlayerData = data.players[data.currPlayerIndex]
			currPlayerData.name = newPlayerName
			currPlayerData.password = newPlayerPassword

			data.currPlayerIndex++
			if (data.currPlayerIndex < data.playerCount) {
				reloadPlayerDataMenu()
			} else {
				binding.playerDataManu.visibility = View.GONE
				startStage(1)
			}

		}



		binding.startStageStartButton.setOnClickListener {

			data.roundNum = 1
			data.buckshot = ArrayList()
			for (player in data.players) {
				player.handcuffLevel = 0
				player.items.clear()
			}

			binding.startStageMenu.visibility = View.GONE
			binding.startRoundMenu.visibility = View.VISIBLE
			reloadStartRoundMenu()

		}



		binding.startRoundStartButton.setOnClickListener {

			giveItems()
			reloadBuckshot()
			for (player in data.players) {
				player.alreadyShotWithLive = false
				player.alreadyTraded = false
			}

			binding.startRoundMenu.visibility = View.GONE
			binding.reloadBuckshotMenu.visibility = View.VISIBLE
			reloadReloadBuckshotMenu()

		}



		binding.reloadBuckshotNextButton.setOnClickListener {

			data.currPlayerIndex = 0

			binding.reloadBuckshotMenu.visibility = View.GONE
			binding.giveItemsMenu.visibility = View.VISIBLE
			reloadGiveItemsMenu()

		}



		binding.giveItemsItemsInnerNextButton.setOnClickListener {

			data.currPlayerIndex++

			if (data.currPlayerIndex < data.playerCount) {
				reloadGiveItemsMenu()
			} else {

				data.currPlayerIndex = 0

				binding.giveItemsMenu.visibility = View.GONE
				binding.nextTurnMenu.visibility = View.VISIBLE
				reloadNextTurnMenu()

			}

		}



		binding.nextTurnGoButton.setOnClickListener {

			val currPlayer = data.players[data.currPlayerIndex]

			if (binding.nextTurnPasswordInput.text.toString() != currPlayer.password) {
				binding.nextTurnErrorLabel.text = getString(R.string.error_password_is_incorrect)
				return@setOnClickListener
			}

			binding.nextTurnMenu.visibility = View.GONE
			binding.mainTurnMenu.visibility = View.VISIBLE
			reloadMainTurnMenu()

		}



		binding.mainTurnMiddlePlayersButton.setOnClickListener {

			binding.mainTurnMenu.visibility = View.GONE
			binding.playersListMenu.visibility = View.VISIBLE
			reloadPlayersListMenu()

		}

		binding.playersListPlayersInnerBackButton.setOnClickListener {

			binding.playersListMenu.visibility = View.GONE
			binding.mainTurnMenu.visibility = View.VISIBLE

		}



	}



	private fun startStage(stageNum: Int) {

		data.stage = stageNum
		when (data.stage) {
			1 -> {
				data.maxLives = 2
				data.maxItems = 4
				data.itemsPerTurn = 2
				data.pointsPerShot = 1
				data.pointsOnWin = 10
				data.minBuckshotShells = 2
				data.maxBuckshotShells = 4
				data.minLivePercent = 0.33
				data.maxLivePercent = 0.66
			}
			2 -> {
				data.maxLives = 3
				data.maxItems = 6
				data.itemsPerTurn = 3
				data.pointsPerShot = 2
				data.pointsOnWin = 20
				data.minBuckshotShells = 4
				data.maxBuckshotShells = 8
				data.minLivePercent = 0.33
				data.maxLivePercent = 0.66
			}
			3 -> {
				data.maxLives = 4
				data.maxItems = 8
				data.itemsPerTurn = 4
				data.pointsPerShot = 3
				data.pointsOnWin = 30
				data.minBuckshotShells = 6
				data.maxBuckshotShells = 8
				data.minLivePercent = 0.33
				data.maxLivePercent = 0.66
			}
		}
		for (i in 0..<data.playerCount) {
			data.players[i].lives = data.maxLives
		}

		binding.startStageMenu.visibility = View.VISIBLE
		@SuppressLint("SetTextI18n")
		binding.startStageLabel.text = "Stage $stageNum"

	}



	private fun reloadBuckshot() {
		val shellCount = Random.nextInt(data.minBuckshotShells, data.maxBuckshotShells + 1)
		val livePercent = Random.nextDouble(data.minLivePercent, data.maxLivePercent)
		val liveShellCount = round(shellCount * livePercent).toInt()
		val blankShellCount = shellCount - liveShellCount
		for (i in 0..<liveShellCount) data.buckshot.add(true)
		for (i in 0..<blankShellCount) data.buckshot.add(false)
		data.buckshot.shuffle()
	}



	private fun giveItems() {
		for (i in 0..<data.playerCount) {
			val currPlayer = data.players[i]
			val itemDeficit = data.maxItems - currPlayer.items.size
			val newItemsCount = min(data.itemsPerTurn, itemDeficit)
			val newItems = Array(newItemsCount) {randomItem()}
			currPlayer.newItems = newItems
			for (item in newItems) {
				currPlayer.items.add(item)
			}
		}
	}

	private fun min(a: Int, b: Int): Int {
		return if (a < b) {a} else {b}
	}


	private fun reloadPlayerDataMenu() {
		@SuppressLint("SetTextI18n")
		binding.playerDataTopLabel.text = "Player ${data.currPlayerIndex + 1}:"
		binding.playerDataNameInput.text = emptyEditableText()
		binding.playerDataPasswordInput.text = emptyEditableText()
		binding.playerDataErrorLabel.text = ""
	}



	private fun reloadStartRoundMenu() {
		@SuppressLint("SetTextI18n")
		binding.startRoundLabel.text = "Round ${data.roundNum}"
	}



	private fun reloadReloadBuckshotMenu() {

		var liveCount = 0
		var blankCount = 0
		for (shell in data.buckshot) {
			if (shell) {
				liveCount++
			} else {
				blankCount++
			}
		}

		@SuppressLint("SetTextI18n")
		binding.reloadBuckshotLabel.text = "The buckshot is loaded with ${pluralize(liveCount, "live", "lives")} and ${pluralize(blankCount, "blank", "blanks")}."

	}



	private fun reloadGiveItemsMenu() {
		val currPlayer = data.players[data.currPlayerIndex]

		@SuppressLint("SetTextI18n")
		binding.giveItemsLabel.text = currPlayer.name + " is given:"

		val innerArea = binding.giveItemsItemsInnerArea

		val nextButton = binding.giveItemsItemsInnerNextButton
		innerArea.removeAllViews()
		innerArea.addView(nextButton)

		val itemViews = currPlayer.newItems.map { item ->
			val itemView = TextView(this)
			itemView.id = View.generateViewId()
			itemView.layoutParams = defaultLayoutParams()

			itemView.text = item.toSingularString()
			itemView.textAlignment = View.TEXT_ALIGNMENT_CENTER
			itemView.textSize = 24F

			innerArea.addView(itemView)
			itemView
		}

		val innerAreaConstraints = ConstraintSet()
		innerAreaConstraints.clone(innerArea)

		var currTopConstraint = innerArea as View
		var topConstraintType = ConstraintSet.TOP
		for (itemView in itemViews) {

			innerAreaConstraints.connect(itemView.id, ConstraintSet.START, innerArea.id, ConstraintSet.START)
			innerAreaConstraints.connect(itemView.id, ConstraintSet.END, innerArea.id, ConstraintSet.END)
			innerAreaConstraints.connect(itemView.id, ConstraintSet.TOP, currTopConstraint.id, topConstraintType)

			currTopConstraint = itemView
			topConstraintType = ConstraintSet.BOTTOM

		}

		innerAreaConstraints.connect(nextButton.id, ConstraintSet.TOP, currTopConstraint.id, topConstraintType)

		innerAreaConstraints.applyTo(innerArea)

	}



	private fun reloadNextTurnMenu() {

		val currPlayer = data.players[data.currPlayerIndex]

		@SuppressLint("SetTextI18n")
		binding.nextTurnLabel.text = currPlayer.name + "'s turn."
		binding.nextTurnPasswordInput.text = emptyEditableText()
		binding.nextTurnPasswordArea.visibility = if (currPlayer.password.isEmpty()) {View.GONE} else {View.VISIBLE}

	}



	private fun reloadMainTurnMenu() {

		val currPlayer = data.players[data.currPlayerIndex]

		@SuppressLint("SetTextI18n")
		binding.mainTurnLabel.text = currPlayer.name + "'s Turn"
		@SuppressLint("SetTextI18n")
		binding.mainTurnItemsBottomLabel.text = "(Max items: " + data.maxItems + ")"

		val innerArea = binding.mainTurnItemsInner

		val maxItemsLabel = binding.mainTurnItemsBottomLabel
		innerArea.removeAllViews()
		innerArea.addView(maxItemsLabel)

		val itemRowViews = currPlayer.items.mapIndexed { i, item ->
			val itemRowView = generateItemRowView(item.toSingularString(), i)
			innerArea.addView(itemRowView)
			itemRowView
		}

		val rowViews = if (currPlayer.items.isEmpty()) {
			val noItemsLabel = TextView(this)
			noItemsLabel.id = View.generateViewId()
			innerArea.addView(noItemsLabel)
			noItemsLabel.text = getString(R.string.no_items)
			noItemsLabel.textSize = 16F
			listOf(noItemsLabel)
		} else {
			itemRowViews
		}

		val innerAreaConstraints = ConstraintSet()
		innerAreaConstraints.clone(innerArea)

		var currTopConstraint = innerArea as View
		var topConstraintType = ConstraintSet.TOP
		for (itemRowView in rowViews) {

			innerAreaConstraints.connect(itemRowView.id, ConstraintSet.START, innerArea.id, ConstraintSet.START)
			innerAreaConstraints.connect(itemRowView.id, ConstraintSet.END, innerArea.id, ConstraintSet.END)
			innerAreaConstraints.connect(itemRowView.id, ConstraintSet.TOP, currTopConstraint.id, topConstraintType)

			currTopConstraint = itemRowView
			topConstraintType = ConstraintSet.BOTTOM

		}

		innerAreaConstraints.connect(binding.mainTurnItemsBottomLabel.id, ConstraintSet.TOP, currTopConstraint.id, topConstraintType)

		innerAreaConstraints.applyTo(innerArea)

	}

	private fun generateItemRowView(itemName: String, index: Int): ConstraintLayout {

		val itemLayout = ConstraintLayout(this)
		itemLayout.id = View.generateViewId()
		val itemLayoutLayout = ConstraintLayout.LayoutParams(0, ConstraintLayout.LayoutParams.WRAP_CONTENT)
		(itemLayoutLayout as MarginLayoutParams).setMargins(0, 0, 32, 0)
		itemLayout.layoutParams = itemLayoutLayout

		val itemLabel = TextView(this)
		itemLabel.id = View.generateViewId()
		itemLabel.layoutParams = defaultLayoutParams()
		itemLabel.text = itemName
		itemLabel.textAlignment = View.TEXT_ALIGNMENT_CENTER
		itemLabel.textSize = 20F
		itemLayout.addView(itemLabel)

		val itemUseButton = Button(this)
		itemUseButton.id = View.generateViewId()
		itemUseButton.layoutParams = defaultLayoutParams()
		itemUseButton.width = 30
		itemUseButton.height = 20
		itemUseButton.text = getString(R.string.use)
		itemLayout.addView(itemUseButton)

		val itemLayoutConstraints = ConstraintSet()
		itemLayoutConstraints.clone(itemLayout)
		itemLayoutConstraints.connect(itemUseButton.id, ConstraintSet.TOP, itemLayout.id, ConstraintSet.TOP)
		itemLayoutConstraints.connect(itemUseButton.id, ConstraintSet.BOTTOM, itemLayout.id, ConstraintSet.BOTTOM)
		itemLayoutConstraints.connect(itemUseButton.id, ConstraintSet.END, itemLayout.id, ConstraintSet.END)
		itemLayoutConstraints.connect(itemLabel.id, ConstraintSet.TOP, itemLayout.id, ConstraintSet.TOP)
		itemLayoutConstraints.connect(itemLabel.id, ConstraintSet.BOTTOM, itemLayout.id, ConstraintSet.BOTTOM)
		itemLayoutConstraints.connect(itemLabel.id, ConstraintSet.START, itemLayout.id, ConstraintSet.START)
		itemLayoutConstraints.connect(itemLabel.id, ConstraintSet.END, itemUseButton.id, ConstraintSet.START)
		itemLayoutConstraints.applyTo(itemLayout)

		return itemLayout
	}



	private fun reloadPlayersListMenu() {

	}



	private fun pluralize(count: Int, singular: String, plural: String): String {
		val unit = if (count == 1) {
			singular
		} else {
			plural
		}
		return "$count $unit"
	}

	private fun emptyEditableText(): Editable  {
		return Editable.Factory().newEditable("")
	}

	private fun defaultLayoutParams(): LayoutParams {
		return ConstraintLayout.LayoutParams(
			ConstraintLayout.LayoutParams.WRAP_CONTENT,
			ConstraintLayout.LayoutParams.WRAP_CONTENT
		)
	}



}



class GameData {

	var playerCount = 0
	var currPlayerIndex = 0
	var players = emptyArray<PlayerData>()

	var stage = 0
	var roundNum = 0

	var maxLives = 0
	var maxItems = 0
	var itemsPerTurn = 0
	var pointsPerShot = 0
	var pointsOnWin = 0

	var buckshot: MutableList<Boolean> = ArrayList()
	var buckshotIsExtended = false
	var minBuckshotShells = 0
	var maxBuckshotShells = 0
	var minLivePercent = 0.0
	var maxLivePercent = 0.0

}



class PlayerData {

	var name = ""
	var password = ""

	var credits = 0

	var lives = 0
	var alreadyShotWithLive = false
	var handcuffLevel = 0
	var items: MutableList<Item> = ArrayList()
	var newItems = emptyArray<Item>()
	var alreadyTraded = false

}



enum class Item {

	CIGARETTES,
	MAGNIFYING_GLASS,
	BEER,
	BARREL_EXTENSION,
	HANDCUFFS,
	UNKNOWN_TICKET,
	GOLD_STAR,
	SILVER_STAR;

	fun toSingularString(): String {
		return when (this) {
			CIGARETTES -> "Cigarettes"
			MAGNIFYING_GLASS -> "A Magnifying Glass"
			BEER -> "A Beer Can"
			BARREL_EXTENSION -> "A Barrel Extension"
			HANDCUFFS -> "Handcuffs"
			UNKNOWN_TICKET -> "An Unknown Ticket"
			GOLD_STAR -> "A Gold Star"
			SILVER_STAR -> "A Silver Star"
		}
	}

}

val itemChooseMultipliers = mapOf(
	Pair("CIGARETTES"      , 1.0),
	Pair("MAGNIFYING_GLASS", 0.6),
	Pair("BEER"            , 1.0),
	Pair("BARREL_EXTENSION", 0.6),
	Pair("HANDCUFFS"       , 1.0),
	Pair("UNKNOWN_TICKET"  , 0.3),
	Pair("GOLD_STAR"       , 1.0),
	Pair("SILVER_STAR"     , 0.6),
)
val itemsChooseMultiplierTotal = run {
	var total = 0.0
	for (item in itemChooseMultipliers) {
		total += item.value
	}
	total
}

fun randomItem(): Item {
	val chosenLocation = Random.nextDouble(itemsChooseMultiplierTotal)
	var currTotal = 0.0
	for (item in itemChooseMultipliers) {
		currTotal += item.value
		if (currTotal > chosenLocation) {
			return Item.valueOf(item.key)
		}
	}
	error("unreachable")
}
