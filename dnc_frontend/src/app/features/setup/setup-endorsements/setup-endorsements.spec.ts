import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SetupEndorsements } from './setup-endorsements';

describe('SetupEndorsements', () => {
  let component: SetupEndorsements;
  let fixture: ComponentFixture<SetupEndorsements>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SetupEndorsements]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SetupEndorsements);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
