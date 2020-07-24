#![allow(non_snake_case)]

use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use MyShip;
use Asteroid;
use Bullet;
use Util;

// ---------------------------------
// ゲーム本体制御
// ---------------------------------
pub enum GameState {
    TITLE,
    START,
    PLAY,
    DEAD,
    GAMEOVER
}

pub struct Game {
    state: GameState,
    score: i32,
    nShips: i32,
    nAsteroids: i32,

    D_ROT: f64,
    MAX_SPEED: f64,

    myShip: Option<MyShip::MyShip>,
    bullets: Vec<Bullet::Bullet>,
    asteroids: Vec<Asteroid::Asteroid>
}

impl Game {
    pub fn new() -> Self {
        return Game {
            state: GameState::START,
            score: 0,
            nShips: 5,
            nAsteroids: 0,
            MAX_SPEED: 15.0,    // 最大速度
            D_ROT: Util::deg2rad(5.0),  // 回転変位
            myShip: None,
            bullets: Vec::new(),
            asteroids: Vec::new()
        };
    }

    pub fn upPressed(&mut self) {
        match self.state {
            GameState::PLAY => {
                if let Some(ref mut myShip) = self.myShip {
                    myShip.updateDir(); // 加速
                }
            }
            _ => {
                // do nothing
            }
        }
    }

    pub fn leftPressed(&mut self) {
        match self.state {
            GameState::PLAY => {
                if let Some(ref mut myShip) = self.myShip {
                    myShip.incrRotation(-self.D_ROT);    // 左回転
                }
            }
            _ => {
                // do nothing
            }
        }
    }

    pub fn rightPressed(&mut self) {
        match self.state {
            GameState::PLAY => {
                if let Some(ref mut myShip) = self.myShip {
                    myShip.incrRotation(self.D_ROT);     // 右回転
                }
            }
            _ => {
                // do nothing
            }
        }
    }

    pub fn spacePressed(&mut self) {
        match self.state {
            GameState::PLAY => {
                if let Some(ref mut myShip) = self.myShip {
                    self.bullets.push(myShip.fire());  // 弾発射
                }
            }
            _ => {
                // do nothing
            }
        }
    }

    pub fn enterPressed(&mut self) {
        match self.state {
            GameState::TITLE => {
                // ゲーム開始

            }
            _ => {

            }
        }
    }

    pub fn escapePressed(&mut self) {
        
    }

    pub fn init(&mut self, width: u32, height: u32) {
        // 宇宙船初期化
        self.myShip = Some(MyShip::MyShip::new(
            (width/2) as f64,   // x
            (height/2) as f64,  // y
            10.0,   // 半径（宇宙船の大きさ）
            0.0,    // 初期回転角
            self.MAX_SPEED   // 最大速さ
        ));
 
        // 弾丸を保持するベクトル
        self.bullets = Vec::new();

        // 小惑星を保持するベクトル
        self.asteroids = Vec::new();
    
        // 小惑星を生成
        self.nAsteroids = 10;
        for _ in 0..self.nAsteroids {
            let asteroid = Asteroid::Asteroid::genRand(
                width,
                height
            );
            self.asteroids.push(asteroid);
        }
    }

    pub fn update(&mut self, canvas: &mut Canvas<Window>, width: u32, height: u32) {
        match self.state {
            GameState::TITLE => {
                // ゲーム開始(Enter)または終了(Escape)まで待機
            }
            GameState::START => {
                // スコアをクリア
                self.score = 0;
                // ゲーム初期化
                self.init(width, height);
                // ゲーム開始
                self.state = GameState::PLAY;
            }
            GameState::PLAY => {
                // ゲームプレイ中
                // clear canvas
                canvas.set_draw_color(Color::RGB(0, 0, 0)); // black
                canvas.clear();
                
                // 宇宙船更新
                if let Some(ref mut myShip) = self.myShip {
                    // update position
                    myShip.updatePos(width, height);
            
                    // draw ship
                    myShip.draw(canvas);
                }
        
                // update bullets
                let wf = (width as i32) as f64;
                let hf = (height as i32) as f64;
                for bullet in &mut self.bullets {
                    if bullet.getValid() {
                        // 有効なら進める
                        let newx = bullet.getX() + bullet.getVx();
                        let newy = bullet.getY() + bullet.getVy();
        
                        if (newx < 0.0) || (newy < 0.0) || (newx > wf) || (newy > hf) {
                            // 画面から外れた．無効にする
                            // （本来は消すべきだが、dropの仕方がよくわからないのでフラグを使って無効にする）
                            bullet.setValid(false);
                        } else {
                            bullet.setX(newx);
                            bullet.setY(newy);
        
                            // 衝突判定
                            let mut newAsteroids = Vec::new();
                            for asteroid in &mut self.asteroids {
                                if asteroid.getValid() && asteroid.hitTest(newx, newy) {
                                    // 小惑星に当たった
                                    asteroid.setValid(false);
                                    bullet.setValid(false);
        
                                    // 小惑星を分裂させる
                                    if asteroid.getSize() >= 6 {
                                        let newSize = asteroid.getSize() / 2;
                                        for _ in 0..3 {
                                            let newAstroid = Asteroid::Asteroid::new(newSize, asteroid.getX(), asteroid.getY());
                                            newAsteroids.push(newAstroid);
                                        }
                                    }
                                }
                            }
        
                            for newAsteroid in newAsteroids {
                                self.asteroids.push(newAsteroid);
                            }
                        }
                    }
                }
        
                // draw bullets
                for bullet in &self.bullets {
                    if bullet.getValid() {
                        bullet.draw(canvas);
                    }
                }
        
                // update & draw asteroids
                for asteroid in &mut self.asteroids {
                    if asteroid.getValid() {
                        asteroid.update(width, height);
                        asteroid.draw(canvas);
                    }
                }
            }
            GameState::DEAD => {
                // 宇宙船破壊された
                self.nShips -= 1;
                if self.nShips <=0 {
                    // 残り宇宙船なし．ゲームオーバー
                    self.state = GameState::GAMEOVER;
                } else {
                    // まだ残りある．リスタート
                    // init();
                    self.state = GameState::PLAY;
                }
            }
            _ => {

            }
        }
    }
}