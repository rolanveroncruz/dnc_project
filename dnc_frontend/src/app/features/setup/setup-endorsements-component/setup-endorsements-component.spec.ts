import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SetupEndorsementsComponent } from './setup-endorsements-component';

describe('SetupEndorsementsComponent', () => {
  let component: SetupEndorsementsComponent;
  let fixture: ComponentFixture<SetupEndorsementsComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SetupEndorsementsComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SetupEndorsementsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
